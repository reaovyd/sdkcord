use serde::{
    Deserialize,
    Serialize,
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

#[derive(Debug, Serialize, Deserialize)]
struct PayloadCommand {
    cmd: String,
    args: Args,
    nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Args {
    pid: u32,
    activity: Activity,
}

#[derive(Debug, Serialize, Deserialize)]
struct Activity {
    state: String,
    details: String,
    assets: Assets,
    #[serde(rename = "type")]
    activity_type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct PayloadDisconnectActivity {
    cmd: String,
    args: EmptyDisconnectArgs,
    nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmptyDisconnectArgs {
    pid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Assets {
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
}

fn main() {
    // let payload = Payload {
    //     nonce: "f48f6176-4afb-4c03-b1b8-d960861f5216".to_string(),
    //     args: PayloadArgs {
    //         client_id: "192741864418312192".to_string(),
    //     },
    //     cmd: "AUTHORIZE".to_string(),
    // };
    // let bytes = serde_json::to_vec(&payload).unwrap();
    // let payload = Payload {
    //     v: 1,
    //     client_id: "1276759902551015485".to_string(),
    // };
    // let payload_disconnect_activity =
    // serde_json::to_vec(&PayloadDisconnectActivity {
    //     cmd: "SET_ACTIVITY".to_string(),
    //     args: EmptyDisconnectArgs {
    //         pid: std::process::id(),
    //     },
    //     nonce: "c9f07f0e-cbf1-4e5a-aa3d-6dceb967968d".to_string(),
    // })
    // .unwrap();
    // let json =
    // serde_json::from_slice::<serde_json::Value>(&
    // payload_disconnect_activity).unwrap(); println!("{:#}", json);
    // let mut bytes_disconnect_activity = (1u32).to_le_bytes().to_vec();
    // bytes_disconnect_activity.extend((payload_disconnect_activity.len() as
    // u32).to_le_bytes()); bytes_disconnect_activity.
    // extend(payload_disconnect_activity);

    // println!("{:?}", bytes_disconnect_activity);
    // println!("{}", num_cpus::get());
    // println!("{}", num_cpus::get_physical());

    // let payload_set_activity = serde_json::to_vec(&PayloadCommand {
    //     cmd: "SET_ACTIVITY".to_string(),
    //     args: Args {
    //         pid: std::process::id(),
    //         activity: Activity {
    //             state: "test".to_string(),
    //             details: "gaming".to_string(),
    //             assets: Assets {
    //                 large_image: "kita_ikuyo".to_string(),
    //                 large_text: "bruh".to_string(),
    //                 small_image: "kita_ikuyo".to_string(),
    //                 small_text: "bruh".to_string(),
    //             },
    //             activity_type: 3,
    //         },
    //     },
    //     nonce: "647d814a-4cf8-4fbb-948f-898abd24f55b".to_string(),
    // })
    // .unwrap();
    // let json =
    // serde_json::from_slice::<serde_json::Value>(&payload_set_activity).
    // unwrap(); println!("{:#}", json);
    // let mut bytes_set_activity = (1u32).to_le_bytes().to_vec();
    // bytes_set_activity.extend((payload_set_activity.len() as
    // u32).to_le_bytes()); bytes_set_activity.extend(payload_set_activity);

    // let payload = serde_json::to_vec(&payload).unwrap();
    // println!(
    //     "{}",
    //     serde_json::from_slice::<serde_json::Value>(&payload).unwrap()
    // );

    // let payload_len = (payload.len() as u32).to_le_bytes();
    // let mut bytes = (0u32).to_le_bytes().to_vec();

    // bytes.extend(payload_len);
    // bytes.extend(payload);
    // println!("{:?}", bytes);

    // for i in 0..10 {
    //     let socket_path =
    // format!("{XDG_RUNTIME_DIR}/{DISCORD_IPC_PREFIX}{i}");     if let
    // Ok(mut stream) = UnixStream::connect(&socket_path).await {
    //         println!("IPC Client connected to {}", socket_path);
    //         let (mut reader, mut writer) = stream.into_split();
    //         println!("Authorizing payload to Discord IPC");
    //         writer.write(&bytes).await;
    //         let mut opcode = [0; 4];
    //         let mut len = [0; 4];

    //         reader.read(&mut opcode).await;
    //         reader.read(&mut len).await;
    //         let len = u32::from_le_bytes(len) as usize;
    //         let mut buf = vec![0; len];

    //         reader.read(&mut buf).await;
    //         println!("{:?}", String::from_utf8_lossy(&buf));

    //         println!("Setting activity here");
    //         writer.write(&bytes_set_activity).await;
    //         println!("Setting activity done");

    //         // let mut opcode = [0; 4];
    //         // let mut len = [0; 4];
    //         // reader.read(&mut opcode).await;
    //         // reader.read(&mut len).await;
    //         // let len = u32::from_le_bytes(len) as usize;
    //         // let mut buf = vec![0; len];
    //         // reader.read(&mut buf).await;
    //         // println!("{:?}", String::from_utf8_lossy(&buf));

    //         println!("removeing activity here");
    //         writer.write(&bytes_disconnect_activity).await;
    //         println!("removing activity done");

    //         // let mut opcode = [0; 4];
    //         // let mut len = [0; 4];
    //         // reader.read(&mut opcode).await;
    //         // reader.read(&mut len).await;
    //         // let len = u32::from_le_bytes(len) as usize;
    //         // let mut buf = vec![0; len];
    //         // reader.read(&mut buf).await;

    //         // println!("{:?}", String::from_utf8_lossy(&buf));
    //         while (true) {}
    //         exit(0);
    //     } else {
    //         eprintln!("Couldn't find a Discord IPC instance from {}!",
    // socket_path);     }
    // }
}
