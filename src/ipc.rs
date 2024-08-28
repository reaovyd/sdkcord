use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};

pub(crate) mod reader;
pub(crate) mod writer;

type IPCWriter = OwnedWriteHalf;
type IPCReader = OwnedReadHalf;
