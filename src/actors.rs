use std::{io, sync::Arc, time::Duration};

use dashmap::DashMap;
use futures::SinkExt;
use kameo::{
    Actor,
    actor::ActorRef,
    error::{BoxError, SendError},
    mailbox::bounded::BoundedMailbox,
    message::{Context, Message, StreamMessage},
};
use thiserror::Error;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::oneshot,
};
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::{
    SerdeProcessingError,
    codec::{Frame, FrameCodec},
    payload::{Event, PayloadResponse, Request},
    pool::{Client, SerdePoolError},
};

const CONNECT_UUID: Uuid = Uuid::from_u128(0xdffa20e1_f231_4792_9684_4b8449823bbd);

#[derive(Actor)]
#[actor(mailbox = bounded(1024))]
pub(crate) struct Coordinator<T: Send + Sync + 'static> {
    writer: ActorRef<Writer<T>>,
    pending_requests: Arc<DashMap<Uuid, oneshot::Sender<PayloadResponse>>>,
}

impl<T> Coordinator<T>
where
    T: Send + Sync + 'static,
{
    pub(crate) fn new(writer: ActorRef<Writer<T>>) -> Self {
        Self {
            writer,
            pending_requests: Arc::new(DashMap::new()),
        }
    }
}

impl<T> Message<(Request, oneshot::Sender<PayloadResponse>)> for Coordinator<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = Result<(), CoordinatorError>;

    async fn handle(
        &mut self,
        msg: (Request, oneshot::Sender<PayloadResponse>),
        _: Context<'_, Self, Self::Reply>,
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

        // NOTE: from client caller, they will get a CoordinatorError response
        // and it won't be pending on the oneshot receiver
        self.writer
            .ask(request)
            .reply_timeout(Duration::from_secs(5))
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
                    SendError::HandlerError(err) => CoordinatorError::RequestFailed(err),
                    SendError::Timeout(req) => CoordinatorError::WriterTimeout(req),
                }
            })?;

        Ok(())
    }
}

impl<T> Message<PayloadResponse> for Coordinator<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = ();

    async fn handle(
        &mut self,
        mut resp: PayloadResponse,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let pending_requests = self.pending_requests.clone();

        tokio::spawn(async move {
            match resp.0.evt.as_ref() {
                Some(Event::Ready) => {
                    // little bit of a hacky way
                    resp.0.nonce = Some(CONNECT_UUID);
                    send_response(&pending_requests, resp);
                }
                None | Some(Event::Error) => {
                    // only Event with a nonce is the Error type has a nonce
                    send_response(&pending_requests, resp);
                }
                _ => {
                    // TODO: send event to some listener...
                }
            }
        });
    }
}

fn send_response(
    pending_requests: &Arc<DashMap<Uuid, oneshot::Sender<PayloadResponse>>>,
    resp: PayloadResponse,
) {
    if let Some(nonce) = resp.0.nonce.as_ref() {
        if let Some((nonce, sender)) = { pending_requests.remove(nonce) } {
            if sender.send(resp).is_err() {
                error!(
                    "nonce id: [{}] failed to send to client; receiver end may have died!",
                    nonce
                );
            } else {
                info!("nonce id: [{}] successfully sent to client", nonce)
            }
        } else {
            error!("nonce cannot be found in pending requests (perhaps client has timed out?)...");
        }
    } else {
        error!("nonce cannot be found in the response...");
    }
}

pub(crate) struct Reader<T, W: Send + Sync + 'static> {
    deserializer_client: Client<Frame, Result<PayloadResponse, SerdeProcessingError>>,
    reader: Option<FramedRead<T, FrameCodec>>,
    coordinator: ActorRef<Coordinator<W>>,
}

impl<T, W> Reader<T, W>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    pub(crate) const fn new(
        deserializer_client: Client<Frame, Result<PayloadResponse, SerdeProcessingError>>,
        reader: FramedRead<T, FrameCodec>,
        coordinator: ActorRef<Coordinator<W>>,
    ) -> Self {
        Self {
            deserializer_client,
            reader: Some(reader),
            coordinator,
        }
    }
}

impl<T, W> Actor for Reader<T, W>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    type Mailbox = BoundedMailbox<Self>;
    fn new_mailbox() -> (
        Self::Mailbox,
        <Self::Mailbox as kameo::mailbox::Mailbox<Self>>::Receiver,
    ) {
        Self::Mailbox::new(512)
    }

    async fn on_start(&mut self, actor_ref: ActorRef<Self>) -> Result<(), BoxError> {
        // TODO: maybe this looks really weird?
        actor_ref.attach_stream(self.reader.take().unwrap(), (), ());
        Ok(())
    }
}

pub(crate) struct Writer<T> {
    serializer_client: Client<Request, Result<Frame, SerdeProcessingError>>,
    writer: FramedWrite<T, FrameCodec>,
}

impl<T> Writer<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
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
    type Mailbox = BoundedMailbox<Self>;

    fn new_mailbox() -> (
        Self::Mailbox,
        <Self::Mailbox as kameo::mailbox::Mailbox<Self>>::Receiver,
    ) {
        Self::Mailbox::new(64)
    }
}

impl<T> Message<Request> for Writer<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    // TODO: add a response here...
    type Reply = Result<(), WriterError>;

    #[instrument(skip(self, msg))]
    async fn handle(
        &mut self,
        msg: Request,
        _: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let frame = self.serializer_client.serialize(msg).await??;
        Ok(self.writer.send(frame).await?)
    }
}

impl<T, W> Message<StreamMessage<Result<Frame, io::Error>, (), ()>> for Reader<T, W>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    type Reply = ();

    #[instrument(skip(self, msg))]
    async fn handle(
        &mut self,
        msg: StreamMessage<Result<Frame, io::Error>, (), ()>,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        match msg {
            StreamMessage::Next(frame) => {
                // NOTE: cloning actorrefs are "cheap" since it clones Arcs or mpsc::Sender
                let coordinator = self.coordinator.clone();
                let deserializer_client = self.deserializer_client.clone();
                tokio::spawn(async move {
                    match frame {
                        Ok(frame) => {
                            let resp = deserializer_client.deserialize(frame).await;
                            match resp {
                                Ok(resp) => {
                                    match resp {
                                        Ok(resp) => {
                                            if let Err(err) = coordinator.tell(resp).await {
                                                error!(
                                                    "failed to send response to coordinator: {}",
                                                    err
                                                );
                                            }
                                        }
                                        Err(err) => {
                                            error!("deserialization operation failed: {}", err);
                                        }
                                    };
                                }
                                Err(err) => {
                                    error!(
                                        "error while sending message for deserialization: {}",
                                        err
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            error!("frame error: {}", err);
                        }
                    };
                });
            }
            StreamMessage::Started(()) => {
                info!("started listening to discord ipc");
            }
            StreamMessage::Finished(()) => {
                info!("stopped listening to discord ipc");
            }
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum CoordinatorError {
    #[error("discord ipc server is unavailable")]
    IpcWriterUnavailable(Option<Request>),
    #[error("request failed")]
    RequestFailed(WriterError),
    #[error("timeout writing to ipc server")]
    WriterTimeout(Option<Request>),
}

#[derive(Debug, Error)]
pub(crate) enum WriterError {
    #[error(transparent)]
    Serialization(#[from] SerdeProcessingError),
    #[error(transparent)]
    Ipc(#[from] io::Error),
    #[error(transparent)]
    SerializationPool(#[from] SerdePoolError),
}
