use std::{io, sync::Arc, time::Duration};

use dashmap::DashMap;
use futures::SinkExt;
use kameo::{
    actor::ActorRef,
    error::{BoxError, SendError},
    mailbox::bounded::BoundedMailbox,
    message::{Context, Message, StreamMessage},
    Actor,
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
    codec::{Frame, FrameCodec},
    payload::{PayloadRequest, PayloadResponse},
    pool::{Client, SerdePoolError},
};

#[derive(Actor)]
#[actor(mailbox = bounded(1024))]
pub(crate) struct Coordinator<T: Send + Sync + 'static> {
    writer: ActorRef<Writer<T>>,
    pending_requests: Arc<DashMap<Uuid, oneshot::Sender<PayloadResponse>>>,
}

impl<T> Message<(PayloadRequest, oneshot::Sender<PayloadResponse>)> for Coordinator<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    type Reply = Result<(), CoordinatorError>;

    async fn handle(
        &mut self,
        msg: (PayloadRequest, oneshot::Sender<PayloadResponse>),
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let request = msg.0;
        let callback = msg.1;
        let nonce = request.0.nonce.unwrap();

        self.pending_requests.insert(nonce, callback);

        // NOTE: from client caller, they will get a CoordinatorError response
        self.writer.ask(request).reply_timeout(Duration::from_secs(5)).await.map_err(|err| {
            self.pending_requests.remove(&nonce);
            match err {
                SendError::ActorNotRunning(req) => {
                    CoordinatorError::IpcWriterUnavailable(Some(req))
                }
                SendError::ActorStopped => CoordinatorError::IpcWriterUnavailable(None),
                SendError::MailboxFull(req) => CoordinatorError::IpcWriterUnavailable(Some(req)),
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
        resp: PayloadResponse,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let pending_requests = self.pending_requests.clone();

        tokio::spawn(async move {
            // if this is an event, the nonce wouldn't be present
            // only responses have nonces
            if resp.0.evt.is_some() && resp.0.nonce.is_none() {
                todo!("send this to some event listener");
                return;
            }

            if resp.0.nonce.is_none() {
                error!("nonce is missing from payload response...");
                return;
            }

            if let Some((nonce, sender)) =
                { pending_requests.remove(resp.0.nonce.as_ref().unwrap()) }
            {
                if sender.send(resp).is_err() {
                    error!(
                        "nonce id: [{}] failed to send to client; receiver end may have died!",
                        nonce
                    );
                } else {
                    info!("nonce id: [{}] successfully sent to client", nonce)
                }
            } else {
                error!("nonce cannot be found in pending requests...");
            }
        });
    }
}

pub(crate) struct Reader<T, W: Send + Sync + 'static> {
    deserializer_client: Client<Frame, PayloadResponse>,
    reader: Option<FramedRead<T, FrameCodec>>,
    coordinator: ActorRef<Coordinator<W>>,
}

impl<T, W> Actor for Reader<T, W>
where
    T: Send + Sync + 'static,
    T: AsyncRead + Unpin,
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
    type Mailbox = BoundedMailbox<Self>;
    fn new_mailbox() -> (Self::Mailbox, <Self::Mailbox as kameo::mailbox::Mailbox<Self>>::Receiver)
    {
        Self::Mailbox::new(512)
    }

    async fn on_start(&mut self, actor_ref: ActorRef<Self>) -> Result<(), BoxError> {
        // TODO: maybe this looks really weird?
        actor_ref.attach_stream(self.reader.take().unwrap(), (), ());
        Ok(())
    }
}

pub(crate) struct Writer<T> {
    serializer_client: Client<PayloadRequest, Frame>,
    writer: FramedWrite<T, FrameCodec>,
}

impl<T> Actor for Writer<T>
where
    T: Send + Sync + 'static,
{
    type Mailbox = BoundedMailbox<Self>;

    fn new_mailbox() -> (Self::Mailbox, <Self::Mailbox as kameo::mailbox::Mailbox<Self>>::Receiver)
    {
        Self::Mailbox::new(64)
    }
}

impl<T> Message<PayloadRequest> for Writer<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    // TODO: add a response here...
    type Reply = Result<(), WriterError>;

    #[instrument(skip(self, msg))]
    async fn handle(
        &mut self,
        msg: PayloadRequest,
        _: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let frame = self.serializer_client.serialize(msg).await?;
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
                                    if let Err(err) = coordinator.tell(resp).await {
                                        error!("{}", err);
                                    }
                                }
                                Err(err) => {
                                    error!("{}", err);
                                }
                            }
                        }
                        Err(err) => {
                            error!("{}", err);
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
    IpcWriterUnavailable(Option<PayloadRequest>),
    #[error("request failed")]
    RequestFailed(WriterError),
    #[error("timeout writing to ipc server")]
    WriterTimeout(Option<PayloadRequest>),
}

#[derive(Debug, Error)]
pub(crate) enum WriterError {
    #[error(transparent)]
    Serialization(#[from] SerdePoolError),
    #[error(transparent)]
    Ipc(#[from] io::Error),
}
