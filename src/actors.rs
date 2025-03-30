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
use tracing::{error, info, instrument, trace};
use uuid::Uuid;

use crate::{
    SerdeProcessingError,
    codec::{Frame, FrameCodec},
    payload::{Event, PayloadResponse, Request},
    pool::{Client, SerdePoolError},
};

const CONNECT_UUID: Uuid = Uuid::from_u128(0xdffa20e1_f231_4792_9684_4b8449823bbd);

#[derive(Debug, Clone)]
pub(crate) struct Coordinator<W> {
    writer: W,
    pending_requests: Arc<DashMap<Uuid, oneshot::Sender<PayloadResponse>>>,
}

impl<T> Actor for Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Error = CoordinatorError;
}

impl<T> Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    pub(crate) fn new(writer: ActorRef<Writer<T>>) -> Self {
        Self {
            writer,
            pending_requests: Arc::new(DashMap::new()),
        }
    }
}

type CoordinatorMessage = (Request, oneshot::Sender<PayloadResponse>);

impl<T> Message<CoordinatorMessage> for Coordinator<ActorRef<Writer<T>>>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = Result<(), CoordinatorError>;

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
                    SendError::HandlerError(err) => CoordinatorError::RequestFailed(err),
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

    async fn handle(
        &mut self,
        mut msg: PayloadResponse,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let pending_requests = self.pending_requests.clone();

        tokio::spawn(async move {
            match msg.0.evt.as_ref() {
                Some(Event::Ready) => {
                    // little bit of a hacky way
                    msg.0.nonce = Some(CONNECT_UUID);
                    send_response(&pending_requests, msg);
                }
                None | Some(Event::Error) => {
                    // only Event with a nonce is the Error type has a nonce
                    send_response(&pending_requests, msg);
                }
                _ => {
                    // TODO: send event to some listener...
                }
            }
        });
    }
}

pub(crate) struct Reader<T, W: Actor> {
    deserializer_client: Client<Frame, Result<PayloadResponse, SerdeProcessingError>>,
    reader: Option<FramedRead<T, FrameCodec>>,
    coordinator: ActorRef<W>,
}

impl<T, W> Reader<T, Coordinator<ActorRef<Writer<W>>>>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
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
    async fn on_start(&mut self, actor_ref: ActorRef<Self>) -> Result<(), Self::Error> {
        // TODO: maybe this looks really weird?
        actor_ref.attach_stream(self.reader.take().unwrap(), (), ());
        Ok(())
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
                info!("started listening to discord ipc");
            }
            StreamMessage::Finished(()) => {
                info!("stopped listening to discord ipc");
            }
        }
    }
}

pub(crate) struct Writer<T> {
    serializer_client: Client<Request, Result<Frame, SerdeProcessingError>>,
    writer: FramedWrite<T, FrameCodec>,
}

impl<T> Writer<T> {
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
}

impl<T> Message<Request> for Writer<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = Result<(), WriterError>;

    #[instrument(skip(self, msg))]
    async fn handle(&mut self, msg: Request, _: &mut Context<Self, Self::Reply>) -> Self::Reply {
        trace!("serializing discord message; msg: {:?}", msg);
        let frame = self.serializer_client.serialize(msg).await??;
        trace!(
            "got response for serializer and sending frame to writer now; frame: {:?}",
            frame
        );
        Ok(self.writer.send(frame).await?)
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
