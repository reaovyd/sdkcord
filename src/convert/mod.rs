use tokio::sync::mpsc::{
    Receiver,
    Sender,
};

type PayloadResponseSender = Sender<()>;
type PayloadResponseReceiver = Receiver<()>;

type PayloadRequestSender = Sender<()>;
type PayloadRequestReceiver = Receiver<()>;

pub(crate) mod deserializer;
pub(crate) mod serializer;
