//! Actors created with the `kameo` crate for handling IPC communication with Discord
//!
//! # Actors
//! We have the following actors:
//! - [Coordinator]: handles the message received from the [crate::client::SdkClient],
//!   sends it to the [Writer], and also processes event and response messages
//!   from the [Reader].
//! - [Writer]: handles the message sent from the client and sends it to the IPC server
//! - [Reader]: handles the message sent from the server and sends it to the [Coordinator]
//!
//! # Communication Flow
//! The coordinator actor is the main actor that handles the messages from the client. The messages
//! that get sent from the client are sent to the writer actor, which then sends it to the IPC
//! server after serialization. When a response is ready to be received from the IPC server, the
//! server sends it to the reader actor, which then sends it to the coordinator actor for
//! processing.

use std::{io, sync::Arc, time::Duration};

use dashmap::DashMap;
use futures::SinkExt;
use kameo::{
    Actor,
    actor::ActorRef,
    error::SendError,
    message::{Context, Message, StreamMessage},
};
use thiserror::Error;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::oneshot,
};
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::{error, instrument, trace};
use uuid::Uuid;

use crate::{
    SerdeProcessingError,
    codec::{Frame, FrameCodec},
    payload::{Event, EventData, PayloadResponse, Request},
    pool::{Client, SerdePoolError},
};

/// Hacky way to get the nonce for the connect event
const CONNECT_UUID: Uuid = Uuid::from_u128(0xdffa20e1_f231_4792_9684_4b8449823bbd);

/// A Coordinator actor
#[derive(Debug, Clone)]
pub(crate) struct Coordinator<W> {
    /// Writer actor reference
    writer: W,
    /// Pending client requests where we map the nonce to the caller
    pending_requests: Arc<DashMap<Uuid, oneshot::Sender<PayloadResponse>>>,
    evt_queue_tx: async_channel::Sender<EventData>,
}

impl<T> Actor for Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Args = Self;
    type Error = CoordinatorError;

    async fn on_start(args: Self::Args, _: ActorRef<Self>) -> Result<Self, Self::Error> {
        Ok(args)
    }
}

impl<T> Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    /// Creates a new Coordinator actor
    pub(crate) fn new(
        writer: ActorRef<Writer<T>>,
        evt_queue_tx: async_channel::Sender<EventData>,
    ) -> Self {
        Self {
            writer,
            pending_requests: Arc::new(DashMap::new()),
            evt_queue_tx,
        }
    }
}

/// CoordinatorMessage alias for the message sent to the Coordinator actor from the client
type CoordinatorMessage = (Request, oneshot::Sender<PayloadResponse>);

impl<T> Message<CoordinatorMessage> for Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = Result<(), CoordinatorError>;

    #[instrument(level = "trace", skip(self))]
    async fn handle(
        &mut self,
        msg: CoordinatorMessage,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let request = msg.0;
        let callback = msg.1;
        let nonce = {
            if let Request::Payload(payload) = &request {
                payload.0.nonce.unwrap()
            } else {
                CONNECT_UUID
            }
        };

        self.pending_requests.insert(nonce, callback);

        self.writer
            .tell(request)
            .mailbox_timeout(Duration::from_secs(5))
            .await
            .map_err(|err| {
                self.pending_requests.remove(&nonce);
                match err {
                    SendError::ActorNotRunning(req) => {
                        CoordinatorError::IpcWriterUnavailable(Some(req))
                    }
                    SendError::ActorStopped => CoordinatorError::IpcWriterUnavailable(None),
                    SendError::MailboxFull(req) => {
                        CoordinatorError::IpcWriterUnavailable(Some(req))
                    }
                    SendError::Timeout(req) => CoordinatorError::WriterTimeout(req),
                }
            })?;

        Ok(())
    }
}

impl<T> Message<PayloadResponse> for Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = ();

    #[instrument(level = "trace", skip(self))]
    async fn handle(
        &mut self,
        mut msg: PayloadResponse,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        match msg.0.evt.as_ref() {
            Some(Event::Ready) => {
                // little bit of a hacky way
                let pending_requests = self.pending_requests.clone();
                msg.0.nonce = Some(CONNECT_UUID);
                send_response(pending_requests, msg);
            }
            None | Some(Event::Error) => {
                // only Event with a nonce is the Error type has a nonce
                let pending_requests = self.pending_requests.clone();
                send_response(pending_requests, msg);
            }
            Some(_evt) => {
                let evt_queue_tx = self.evt_queue_tx.clone();
                tokio::spawn(async move {
                    let evt_data = EventData::from(msg.0.data.unwrap());
                    evt_queue_tx
                        .send(evt_data)
                        .await
                        .expect("channel closed...");
                });
            }
        }
    }
}

/// Reader actor for handling messages from the IPC server
pub(crate) struct Reader<T, W: Actor> {
    /// Client for deserializing the frame from IPC server
    deserializer_client: Client<Frame, Result<PayloadResponse, SerdeProcessingError>>,
    /// Reader reading the frame in from the IPC server
    reader: Option<FramedRead<T, FrameCodec>>,
    /// Coordinator actor reference to send frame to
    coordinator: ActorRef<W>,
}

impl<T, W> Reader<T, Coordinator<ActorRef<Writer<W>>>>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    /// Create a new [Reader] actor
    pub(crate) const fn new(
        deserializer_client: Client<Frame, Result<PayloadResponse, SerdeProcessingError>>,
        reader: FramedRead<T, FrameCodec>,
        coordinator: ActorRef<Coordinator<ActorRef<Writer<W>>>>,
    ) -> Self {
        Self {
            deserializer_client,
            reader: Some(reader),
            coordinator,
        }
    }
}

impl<T, W> Actor for Reader<T, Coordinator<ActorRef<Writer<W>>>>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    type Error = ();

    type Args = Self;

    #[instrument(level = "trace", skip(args, actor_ref))]
    async fn on_start(
        mut args: Self::Args,
        actor_ref: ActorRef<Self>,
    ) -> Result<Self, Self::Error> {
        actor_ref.attach_stream(args.reader.take().unwrap(), (), ());
        Ok(args)
    }
}

impl<T, W> Message<StreamMessage<Result<Frame, io::Error>, (), ()>>
    for Reader<T, Coordinator<ActorRef<Writer<W>>>>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    type Reply = ();

    #[instrument(level = "trace", skip(self))]
    async fn handle(
        &mut self,
        msg: StreamMessage<Result<Frame, io::Error>, (), ()>,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        match msg {
            StreamMessage::Next(frame) => {
                // NOTE: cloning actorrefs are "cheap" since it clones Arcs or mpsc::Sender
                let coordinator = self.coordinator.clone();
                let deserializer_client = self.deserializer_client.clone();
                tokio::spawn(async move {
                    match frame {
                        Ok(frame) => {
                            process_stream_message_frame(frame, coordinator, deserializer_client)
                                .await;
                        }
                        Err(err) => {
                            error!("frame error: {}", err);
                        }
                    };
                });
            }
            StreamMessage::Started(()) => {
                trace!("started listening to discord ipc");
            }
            StreamMessage::Finished(()) => {
                trace!("stopped listening to discord ipc");
            }
        }
    }
}

/// Writer actor for handling messages to be written to the IPC server
pub(crate) struct Writer<T> {
    /// Client for serializing the message to be sent to the IPC server
    serializer_client: Client<Request, Result<Frame, SerdeProcessingError>>,
    /// Writer writing the frame out to the IPC server
    writer: FramedWrite<T, FrameCodec>,
}

impl<T> Writer<T> {
    /// Create a new [Writer] actor
    pub(crate) const fn new(
        serializer_client: Client<Request, Result<Frame, SerdeProcessingError>>,
        writer: FramedWrite<T, FrameCodec>,
    ) -> Self {
        Self {
            serializer_client,
            writer,
        }
    }
}

impl<T> Actor for Writer<T>
where
    T: Send + Sync + 'static,
{
    type Error = WriterError;

    type Args = Self;

    async fn on_start(args: Self::Args, _: ActorRef<Self>) -> Result<Self, Self::Error> {
        Ok(args)
    }
}

impl<T> Message<Request> for Writer<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = Result<(), WriterError>;

    #[instrument(level = "trace", skip(self))]
    async fn handle(&mut self, msg: Request, _: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let frame = self.serializer_client.serialize(msg).await??;
        trace!(
            "got response for serializer and sending frame to writer now; frame: {:?}",
            frame
        );
        Ok(self.writer.send(frame).await?)
    }
}

/// Send a response back to the client
#[instrument(level = "trace", skip(pending_requests))]
fn send_response(
    pending_requests: Arc<DashMap<Uuid, oneshot::Sender<PayloadResponse>>>,
    resp: PayloadResponse,
) {
    tokio::spawn(async move {
        if let Some(nonce) = resp.0.nonce.as_ref() {
            if let Some((nonce, sender)) = { pending_requests.remove(nonce) } {
                if sender.send(resp).is_err() {
                    error!(
                        "nonce id: [{}] failed to send to client; receiver end may have died!",
                        nonce
                    );
                } else {
                    trace!("nonce id: [{}] successfully sent to client", nonce)
                }
            } else {
                error!(
                    "nonce cannot be found in pending requests (perhaps client has timed out?)..."
                );
            }
        } else {
            error!("nonce cannot be found in the response...");
        }
    });
}

/// Process the message read from the IPC server
#[instrument(level = "trace", skip(coordinator, deserializer_client))]
async fn process_stream_message_frame<W>(
    frame: Frame,
    coordinator: ActorRef<Coordinator<ActorRef<Writer<W>>>>,
    deserializer_client: Client<Frame, Result<PayloadResponse, SerdeProcessingError>>,
) where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    match deserializer_client.deserialize(frame).await {
        Ok(Ok(resp)) => {
            if let Err(err) = coordinator.tell(resp).await {
                error!("failed to send response to coordinator: {}", err);
            }
        }
        Ok(Err(err)) => error!("deserialization operation failed: {}", err),
        Err(err) => error!("error while sending message for deserialization: {}", err),
    }
}

/// Error types for what happens within the [`Coordinator`] actor
#[derive(Debug, Error)]
pub(crate) enum CoordinatorError {
    /// Writing to the IPC server failed
    #[error("discord ipc server is unavailable")]
    IpcWriterUnavailable(Option<Request>),
    /// The reques timed out
    #[error("timeout writing to ipc server")]
    WriterTimeout(Option<Request>),
}

/// Error types for what happens within the [`Writer`] actor
#[derive(Debug, Error)]
pub(crate) enum WriterError {
    /// Serialization of the request failed
    #[error(transparent)]
    Serialization(#[from] SerdeProcessingError),
    /// The request sent to the IPC server failed
    #[error(transparent)]
    Ipc(#[from] io::Error),
    /// The request sent to the serialization pool failed
    #[error(transparent)]
    SerializationPool(#[from] SerdePoolError),
}
