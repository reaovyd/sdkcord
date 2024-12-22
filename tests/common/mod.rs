use ipccord::payload::{common::opcode::Opcode, request_builder::PayloadRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

const XDG_RUNTIME_DIR: &str = env!("XDG_RUNTIME_DIR");
const DISCORD_IPC_PREFIX: &str = "discord-ipc-";
const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
struct PayloadReady<'a> {
    v: u32,
    client_id: &'a str,
}

// TODO: i'll come up with an actual client with typestate control
#[derive(Debug)]
pub struct RawDiscordIpcClient(UnixStream);

impl RawDiscordIpcClient {
    pub async fn connect(client_id: &str) -> Self {
        for port_id in 0..10 {
            let socket_path = format!("{}/{}{}", XDG_RUNTIME_DIR, DISCORD_IPC_PREFIX, port_id);
            if let Ok(mut stream) = UnixStream::connect(socket_path).await {
                let payload = PayloadReady { v: PROTOCOL_VERSION, client_id };
                let bytes = serde_json::to_vec(&payload).unwrap();
                let bytes = pack_bytes(&bytes, &Opcode::Frame);
                let res = send(&mut stream, &bytes).await;
                println!("{:?}", res);
                let evt = res.get("evt").unwrap();
                if evt == "READY" {
                    return Self(stream);
                } else {
                    panic!("Failed to connect to Discord IPC: {:?}", res);
                }
            }
        }
        panic!("Failed to connect to Discord IPC after 10 attempts");
    }

    pub async fn send(&mut self, req: &PayloadRequest) -> Value {
        send(&mut self.0, &serde_json::to_vec(&req).unwrap()).await
    }
}

async fn send(stream: &mut UnixStream, bytes: &[u8]) -> Value {
    stream.write_all(bytes).await.unwrap();
    let mut opcode = [0u8; size_of::<Opcode>()];
    let mut len = [0u8; size_of::<u32>()];
    stream.read_exact(&mut opcode).await.unwrap();
    stream.read_exact(&mut len).await.unwrap();
    let len = u32::from_le_bytes(len) as usize;
    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload).await.unwrap();
    serde_json::from_slice::<Value>(&payload).unwrap()
}

fn pack_bytes(bytes: &[u8], opcode: &Opcode) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size_of_val(opcode) + size_of::<u32>() + bytes.len());
    let opcode = (*opcode as u32).to_le_bytes();
    let len = (bytes.len() as u32).to_le_bytes();
    buf.extend_from_slice(&opcode);
    buf.extend_from_slice(&len);
    buf.extend_from_slice(bytes);
    buf
}
