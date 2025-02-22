use std::io;

use futures::SinkExt;
use kameo::{
    actor::ActorRef,
    error::BoxError,
    mailbox::bounded::BoundedMailbox,
    message::{Context, Message, StreamMessage},
    Actor,
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::{error, info, instrument};

use crate::{
    codec::{Frame, FrameCodec},
    payload::{PayloadRequest, PayloadResponse},
    pool::Client,
};

#[derive(Actor)]
#[actor(mailbox = bounded(1024))]
pub(crate) struct Coordinator<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    writer: ActorRef<Writer<T>>,
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
        // TODO:
        // 1. pull nonce out of msg
        // 2. remove out of concurrent map and get oneshot
        // 3. oneshot send back payload response.
        let nonce = resp.0.nonce;
        todo!()
    }
}

pub(crate) struct Reader<T, W>
where
    W: Send + Sync + 'static,
    W: AsyncWrite + Unpin,
{
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

#[derive(Actor)]
#[actor(mailbox = bounded(64))]
pub(crate) struct Writer<T: Send + Sync + 'static> {
    serializer_client: Client<PayloadRequest, Frame>,
    writer: FramedWrite<T, FrameCodec>,
}

impl<T> Message<PayloadRequest> for Writer<T>
where
    T: Send + Sync + 'static,
    T: AsyncWrite + Unpin,
{
    // TODO: add a response here...
    type Reply = Result<(), ()>;

    #[instrument(skip(self, msg))]
    async fn handle(
        &mut self,
        msg: PayloadRequest,
        _: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let payload_resp = self.serializer_client.send(msg).await;
        match payload_resp {
            Ok(frame) => {
                self.writer.send(frame).await.expect("failed to write to ipc...");
            }
            Err(err) => {
                error!("error when sending request for serialization: {}", err);
            }
        }
        todo!()
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
                match frame {
                    Ok(frame) => {
                        let payload_resp = self.deserializer_client.send(frame).await;
                        match payload_resp {
                            Ok(payload_resp) => {
                                // TODO: send response to a coordinator response pool when we're ready
                                self.coordinator.tell(payload_resp);
                            }
                            Err(err) => {
                                error!("error when sending frame for deserialization: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("error when reading frame from discord ipc: {}", err);
                    }
                };
            }
            StreamMessage::Started(()) => {
                info!("started listening to discord ipc")
            }
            StreamMessage::Finished(()) => {
                info!("stopped listening to discord ipc")
            }
        }
    }
}
