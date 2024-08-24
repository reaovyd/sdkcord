use std::process::exit;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

const XDG_RUNTIME_DIR: &str = env!("XDG_RUNTIME_DIR");
const DISCORD_IPC_PREFIX: &str = "discord-ipc-";
const JSON_STRING: &str = r#"
{
  "nonce": "f48f6176-4afb-4c03-b1b8-d960861f5216",
  "args": {
    "client_id": "192741864418312192",
    "scopes": ["rpc", "identify"]
  },
  "cmd": "AUTHORIZE"
}
"#;

// #[derive(Debug, Serialize, Deserialize)]
// struct PayloadArgs {
//     client_id: String,
// }

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    v: usize,
    client_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // let payload = Payload {
    //     nonce: "f48f6176-4afb-4c03-b1b8-d960861f5216".to_string(),
    //     args: PayloadArgs {
    //         client_id: "192741864418312192".to_string(),
    //     },
    //     cmd: "AUTHORIZE".to_string(),
    // };
    // let bytes = serde_json::to_vec(&payload).unwrap();
    let payload = Payload {
        v: 1,
        client_id: "1276759902551015485".to_string(),
    };
    let payload = serde_json::to_vec(&payload).unwrap();
    println!(
        "{}",
        serde_json::from_slice::<serde_json::Value>(&payload).unwrap()
    );

    let payload_len = (payload.len() as u32).to_le_bytes();
    let mut bytes = vec![0, 0, 0, 0];

    bytes.extend(payload_len);
    bytes.extend(payload);
    println!("{:?}", bytes);

    for i in 0..10 {
        let socket_path = format!("{XDG_RUNTIME_DIR}/{DISCORD_IPC_PREFIX}{i}");
        if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
            println!("IPC Client connected to {}", socket_path);
            let (mut reader, mut writer) = stream.into_split();
            writer.write(&bytes).await;
            let mut buf = vec![0; 256];
            reader.read_to_end(&mut buf).await;
            println!("{:?}", String::from_utf8(buf));
            exit(0);
        } else {
            eprintln!("Couldn't find a Discord IPC instance from {}!", socket_path);
        }
    }
    Ok(())
}
