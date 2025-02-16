use futures::SinkExt;
use kameo::message::{Message, StreamMessage};
use tokio::net::unix::OwnedWriteHalf;
use tokio_util::codec::FramedWrite;
use tracing::{error, info, instrument, warn};

use crate::{
    codec::{Frame, FrameCodec},
    payload::{PayloadRequest, PayloadResponse},
    pool::Client,
};

#[derive(Debug, kameo::Actor)]
#[actor(mailbox = bounded(512))]
pub(crate) struct Reader {
    deserializer_client: Client<Frame, PayloadResponse>,
}

#[derive(Debug, kameo::Actor)]
#[actor(mailbox = bounded(64))]
pub(crate) struct Coordinator {
    serializer_client: Client<PayloadRequest, Frame>,
    writer: FramedWrite<OwnedWriteHalf, FrameCodec>,
}

impl Message<PayloadRequest> for Coordinator {
    type Reply = Result<(), ()>;

    #[instrument(skip(self, msg))]
    async fn handle(
        &mut self,
        msg: PayloadRequest,
        _: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        // store the nonce inside a concurrent shared hashmap
        // hashmap will contain oneshot sender so we can send back response to user
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

impl Message<StreamMessage<Frame, (), ()>> for Reader {
    type Reply = ();

    #[instrument(skip(self, msg))]
    async fn handle(
        &mut self,
        msg: StreamMessage<Frame, (), ()>,
        _: kameo::message::Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        match msg {
            StreamMessage::Next(frame) => {
                let payload_resp = self.deserializer_client.send(frame).await;
                match payload_resp {
                    Ok(ok) => {
                        // TODO: send this to a coordinator response pool when we're ready
                        todo!("`tell` this to a coordinator response pool :))")
                    }
                    Err(err) => {
                        error!("error when sending frame for deserialization: {}", err);
                    }
                }
            }
            StreamMessage::Started(()) => {
                info!("started listening to discord ipc")
            }
            StreamMessage::Finished(()) => {
                warn!("stopped listening to discord ipc")
            }
        }
    }
}
