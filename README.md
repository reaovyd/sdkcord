## sdkcord
A Rust SDK to communicate with the Discord process on your local machine.

## Quick Start
- To get started quickly, you can get a client quickly with the `sdkcord::spawn_client` method, which provides an `SdkClient`. 
- The `SdkClient` provides a `connect` method which must be called initially before sending any requests.

The following example is from `examples/basic.rs`:
```rust
use anyhow::Result;
use sdkcord::{
    client::spawn_client,
    payload::{GetChannelArgs, PayloadRequest, common::channel::ChannelId},
};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let client = spawn_client().await?;
    let client = client.connect("your_client_id").await?;
    let request = PayloadRequest::builder()
        .request(GetChannelArgs(
            ChannelId::builder()
                .channel_id("some_channel_id")
                .build(),
        ))
        .build();
    let response = client.send_request(request).await?;
    println!("{:?}", response);
    Ok(())
}
```
Replace the `your_client_id` with your own client ID and replace the `some_channel_id` with an actual `channel_id`. 

## License
`sdkcord` is dual-licensed under [MIT](https://github.com/reaovyd/sdkcord/blob/main/LICENSE-MIT) or [Apache License Version 2.0](https://github.com/reaovyd/sdkcord/blob/main/LICENSE-APACHE)
