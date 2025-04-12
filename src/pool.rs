//! Serialization and Deserialization Pool
//!
//! This is used to create dedicated threads for procesing serialization and deserialization as
//! they can be expensive operations and can block the main [Tokio][tokio] thread pool
//!
//! An interface is exposed to other parts of the library through the [Client] type to perform the
//! serialization and deserialization operations
use std::{str::FromStr, thread};

use async_channel::Sender;
use bon::builder;
use bytes::Bytes;
use serde_json::Value;
use thiserror::Error;
use tokio::sync::oneshot::{Sender as OneshotSender, error::RecvError};
use tracing::{error, instrument};
use uuid::Uuid;

use crate::{
    codec::Frame,
    payload::{
        AuthenticateData, AuthorizeData, ChannelCreateData, Command, Data, ErrorData, Event,
        GetChannelData, GetChannelsData, GetGuildData, GetGuildsData, GetSelectedVoiceChannelData,
        GetVoiceSettingsData, GuildCreateData, GuildStatusData, Payload, PayloadResponse,
        ReadyData, Request, SelectTextChannelData, SelectVoiceChannelData, SetActivityData,
        SetUserVoiceSettingsData, SetVoiceSettingsData, SubscribeData, UnsubscribeData,
        VoiceChannelSelectData, VoiceConnectionStatusData, VoiceStateCreateData,
        VoiceStateDeleteData, VoiceStateUpdateData, common::opcode::Opcode,
    },
};

/// Generic Serde Client
///
/// Used to send and receive either a serialization response or a deserialization response on a
/// pool of dedicated threads that is separate from the [Tokio][tokio] thread pool
#[derive(Debug, Clone)]
pub(crate) struct Client<M, R>(Sender<(M, OneshotSender<R>)>);

impl<M, R> Client<M, R>
where
    M: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    /// Deserialize a message
    ///
    /// # Errors
    /// A [SerdePoolError] is returned on two conditions:
    /// 1. If the pool could not receive the response
    /// 2. The client cannot be receive the response because the oneshot channel sender has been
    ///    dropped
    #[inline(always)]
    pub(crate) async fn deserialize(&self, data: M) -> Result<R, SerdePoolError> {
        self.send(data).await
    }
    /// Serialize a message
    ///
    /// # Errors
    /// A [SerdePoolError] is returned on two conditions:
    /// 1. If the pool could not receive the response
    /// 2. The client cannot be receive the response because the oneshot channel sender has been
    ///    dropped
    #[inline(always)]
    pub(crate) async fn serialize(&self, data: M) -> Result<R, SerdePoolError> {
        self.send(data).await
    }
    /// Helper method used by [serialize] and [deserialize] to send a message to the pool
    ///
    /// # Errors
    /// A [SerdePoolError] is returned on two conditions:
    /// 1. If the pool could not receive the response
    /// 2. The client cannot be receive the response because the oneshot channel sender has been
    ///    dropped
    #[inline(always)]
    async fn send(&self, data: M) -> Result<R, SerdePoolError> {
        let (sndr, recv) = tokio::sync::oneshot::channel();
        self.0
            .send((data, sndr))
            .await
            .map_err(|_| SerdePoolError::PoolSend)?;
        recv.await.map_err(SerdePoolError::OneshotRecv)
    }
}

/// Create a pool of threads to handle serialization or deserialization and return a [Client] to
/// either deserialize or serialize the provided message.
///
/// - `num_threads` is the number of threads to spawn in the pool
/// - `op`: `deserialize` or `serialize` operation
/// - `channel_buffer` is the number of messages that can be stored in the async channel
#[builder]
#[instrument(level = "trace", skip(op))]
pub(crate) fn spawn_pool<F, M, R>(num_threads: u8, op: F, channel_buffer: usize) -> Client<M, R>
where
    F: Fn(&M) -> R + Send + Clone + 'static,
    M: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    let (sndr, recv) = async_channel::bounded::<(M, OneshotSender<R>)>(channel_buffer);
    thread::spawn(move || {
        let handlers = (0..num_threads).map(|_| {
            let op = op.clone();
            let recv = recv.clone();
            thread::spawn(move || loop {
                if let Ok((data, sender)) = recv.recv_blocking() {
                    if sender.send(op(&data)).is_err() {
                        error!("sender failed to send job response data! the receiving task may have likely died before it received the value.");
                    }
                } else {
                    error!("channel is closed. closing receiver end and exiting");
                    recv.close();
                    break;
                }
            })
        }).collect::<Vec<_>>();
        for thread in handlers {
            thread.join().unwrap();
        }
    });
    Client(sndr)
}

/// Serialize a request and creates a [Frame] out of it
///
/// # Errors
/// [SerdeProcessingError] is returned if serialization fails
#[inline(always)]
pub(crate) fn serialize(request: &Request) -> Result<Frame, SerdeProcessingError> {
    let (opcode, payload) = {
        match request {
            Request::Connect(connect_request) => (
                Opcode::Handshake,
                Bytes::from(
                    serde_json::to_vec(connect_request)
                        .map_err(|err| SerdeProcessingError::Serialization(err.to_string()))?,
                ),
            ),
            Request::Payload(payload_request) => (
                Opcode::Frame,
                Bytes::from(
                    serde_json::to_vec(payload_request)
                        .map_err(|err| SerdeProcessingError::Serialization(err.to_string()))?,
                ),
            ),
        }
    };
    Ok(Frame {
        opcode,
        len: payload.len() as u32,
        payload,
    })
}

/// Deserialize a request and creates a [PayloadResponse] out of it
///
/// # Errors
/// [SerdeProcessingError] is returned if deserialization fails
pub(crate) fn deserialize(frame: &Frame) -> Result<PayloadResponse, SerdeProcessingError> {
    let mut payload = serde_json::from_slice::<Value>(&frame.payload)
        .map_err(|err| SerdeProcessingError::Deserialization(err.to_string()))?;
    let cmd = payload
        .get("cmd")
        .ok_or_else(|| {
            SerdeProcessingError::Deserialization(
                "failed to get the command from the payload".to_string(),
            )
        })?
        .as_str()
        .ok_or_else(|| {
            SerdeProcessingError::Deserialization(
                "failed to convert the command to a string".to_string(),
            )
        })?;

    let cmd = Command::from_str(cmd)
        .map_err(|err| SerdeProcessingError::Deserialization(err.to_string()))?;

    let nonce = payload
        .get("nonce")
        .and_then(|nonce| nonce.as_str())
        .map(|nonce_str| {
            Uuid::from_str(nonce_str)
                .map_err(|err| SerdeProcessingError::Deserialization(err.to_string()))
        })
        .transpose()?;
    let evt = payload
        .get("evt")
        .and_then(|evt| evt.as_str())
        .map(|evt_str| {
            Event::from_str(evt_str)
                .map_err(|err| SerdeProcessingError::Deserialization(err.to_string()))
        })
        .transpose()?;
    let data = {
        match (evt, cmd) {
            (Some(Event::Error), _) => {
                deserialize_data!(payload, Error)
            }
            (Some(Event::Ready), _) => {
                deserialize_data!(payload, Ready)
            }
            (Some(evt), Command::Dispatch) => match evt {
                Event::GuildStatus => {
                    deserialize_data!(payload, GuildStatus)
                }
                Event::GuildCreate => {
                    deserialize_data!(payload, GuildCreate)
                }
                Event::ChannelCreate => {
                    deserialize_data!(payload, ChannelCreate)
                }
                Event::VoiceChannelSelect => {
                    deserialize_data!(payload, VoiceChannelSelect)
                }
                Event::VoiceStateCreate => {
                    deserialize_data!(payload, VoiceStateCreate)
                }
                Event::VoiceStateUpdate => {
                    deserialize_data!(payload, VoiceStateUpdate)
                }
                Event::VoiceStateDelete => {
                    deserialize_data!(payload, VoiceStateDelete)
                }
                Event::VoiceConnectionStatus => {
                    deserialize_data!(payload, VoiceConnectionStatus)
                }
                Event::SpeakingStart => todo!(),
                Event::SpeakingStop => todo!(),
                Event::MessageCreate => todo!(),
                Event::MessageUpdate => todo!(),
                Event::MessageDelete => todo!(),
                Event::NotificationCreate => todo!(),
                Event::ActivityJoin => todo!(),
                Event::ActivitySpectate => todo!(),
                Event::ActivityJoinRequest => todo!(),
                _ => {
                    panic!(
                        "this should not happen since Event::Ready and Event::Error are covered previously"
                    )
                }
            },
            (None, Command::Authorize) => {
                deserialize_data!(payload, Authorize)
            }
            (None, Command::Authenticate) => {
                deserialize_data!(payload, Authenticate)
            }
            (None, Command::GetGuild) => {
                deserialize_data!(payload, GetGuild)
            }
            (None, Command::GetGuilds) => {
                deserialize_data!(payload, GetGuilds)
            }
            (None, Command::GetChannel) => {
                deserialize_data!(payload, GetChannel)
            }
            (None, Command::SelectVoiceChannel) => {
                deserialize_data!(payload, SelectVoiceChannel)
            }
            (None, Command::GetSelectedVoiceChannel) => {
                deserialize_data!(payload, GetSelectedVoiceChannel)
            }
            (None, Command::SelectTextChannel) => {
                deserialize_data!(payload, SelectTextChannel)
            }
            (None, Command::Subscribe) => {
                deserialize_data!(payload, Subscribe)
            }
            (None, Command::Unsubscribe) => {
                deserialize_data!(payload, Unsubscribe)
            }
            (None, Command::SetUserVoiceSettings) => {
                deserialize_data!(payload, SetUserVoiceSettings)
            }
            (None, Command::SetVoiceSettings) => {
                deserialize_data!(payload, SetVoiceSettings)
            }
            (None, Command::GetVoiceSettings) => {
                deserialize_data!(payload, GetVoiceSettings)
            }
            (None, Command::SetActivity) => {
                deserialize_data!(payload, SetActivity)
            }
            (None, Command::GetChannels) => {
                deserialize_data!(payload, GetChannels)
            }
            (_, _) => {
                todo!()
            }
        }
    };

    Ok(PayloadResponse(Payload {
        cmd,
        nonce,
        evt,
        data,
        args: None,
    }))
}

/// Pool Error is returned when sending or receiving a message to or from the pool fails
#[derive(Debug, Clone, Error)]
pub(crate) enum SerdePoolError {
    /// Error when pool could not receive the message
    #[error("the pool could not receive the response as pool channel has been closed")]
    PoolSend,
    /// Error when oneshot channel is killed and client cannot receive the response
    #[error("the oneshot channel sender has been killed and channel is closed without messages")]
    OneshotRecv(#[from] RecvError),
}

/// Error that occurs when serialization or deserialization fails
#[derive(Debug, Clone, Error)]
pub enum SerdeProcessingError {
    /// Error that occurs when serialization
    #[error("serialization failed: {0}")]
    Serialization(String),
    /// Error that occurs when deserialization
    #[error("deserialization failed: {0}")]
    Deserialization(String),
}

mod macros {
    macro_rules! deserialize_data {
        ($payload: expr, $data_type: ident) => {
            paste::paste! {
                $payload
                    .get_mut("data")
                    .map(|val| {
                        serde_json::from_value::<[<$data_type Data>]>(val.take())
                            .map_err(|err| SerdeProcessingError::Deserialization(err.to_string()))
                    })
                    .transpose()?
                    .map(|data| Data::$data_type(Box::new(data)))
            }
        };
    }
    pub(super) use deserialize_data;
}

use macros::deserialize_data;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bytes::Bytes;
    use pretty_assertions::assert_eq;
    use tokio::time::sleep;

    use crate::payload::{
        ConnectRequest, GetChannelArgs, PayloadRequest,
        common::{channel::ChannelId, opcode::Opcode},
    };

    use super::{Frame, Request, serialize, spawn_pool};

    #[inline(always)]
    const fn op(num: &u32) -> u32 {
        *num + 1
    }

    #[inline(always)]
    const fn op_throws_error(_num: &u32) -> u32 {
        panic!("error");
    }

    #[tokio::test]
    async fn test_pool_spawn_op() {
        let sender = spawn_pool().num_threads(8).op(op).channel_buffer(8).call();
        let handlers = (0..4)
            .map(|_| {
                let tx = sender.clone();
                tokio::spawn(async move { tx.send(3).await.unwrap() })
            })
            .collect::<Vec<_>>();
        for handler in handlers {
            assert_eq!(4, handler.await.unwrap());
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_pool_spawn_op_throws_error() {
        let sender = spawn_pool()
            .num_threads(8)
            .op(op_throws_error)
            .channel_buffer(8)
            .call();
        sender.send(12).await.unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_pool_spawn_dies() {
        let sender = spawn_pool().num_threads(0).op(op).channel_buffer(8).call();
        // necessary to avoid race where sender sends before the pool dies
        sleep(Duration::from_secs(1)).await;
        sender.send(12).await.unwrap();
    }

    #[test]
    fn test_serialize_connect() {
        let connect_request = Request::Connect(ConnectRequest::new("abcdef".to_string()));
        let expected_frame = Frame {
            opcode: Opcode::Handshake,
            len: 28,
            payload: Bytes::from_static(b"{\"v\":1,\"client_id\":\"abcdef\"}"),
        };
        let actual_frame = serialize(&connect_request).unwrap();
        assert_eq!(expected_frame, actual_frame);
    }

    #[test]
    fn test_serialize_payload_request() {
        let connect_request = Request::Payload(
            PayloadRequest::builder()
                .request(GetChannelArgs(
                    ChannelId::builder().channel_id("123").build(),
                ))
                .build(),
        );
        let expected_frame = Frame {
            opcode: Opcode::Frame,
            len: 96,
            payload: Bytes::from_static(b"{\"cmd\":\"GET_CHANNEL\",\"nonce\":\"130bf161-5978-4368-b659-ae6b8de6e276\",\"args\":{\"channel_id\":\"123\"}}"),
        };
        let actual_frame = serialize(&connect_request).unwrap();
        assert_eq!(expected_frame.opcode, actual_frame.opcode);
        assert_eq!(expected_frame.len, actual_frame.len);
    }
}
