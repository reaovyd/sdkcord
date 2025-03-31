# sdkcord
A Rust SDK to communicate with the Discord process on your local machine.

[![Build Status][actions-badge]][actions-url]
[actions-badge]: https://github.com/reaovyd/sdkcord/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/reaovyd/sdkcord/actions/workflows/ci.yml

## Quick Start
- To get started quickly, you can get a client quickly with the `sdkcord::spawn_client` method, which provides an `SdkClient`. 
- The `SdkClient` provides a `connect` method which must be called initially before sending any requests.

The following example is from [`examples/basic.rs`](https://github.com/reaovyd/sdkcord/blob/main/examples/basic.rs):
```rust no_run
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

## RPC Caveat
`sdkcord` is based on the [RPC documentation](https://discord.com/developers/docs/topics/rpc) provided by Discord. However,
there is a caveat with implementing the `sdkcord` based off on the RPC documentation: the RPC documentation
isn't exactly up-to-date with what's provided in the IPC socket for the Discord client. So, a lot of the response
types defined in the RPC documentation can be wildly different from what's actually provided in the IPC socket.

### New fields
There have been cases where the response types sent from Discord can add a new field, in which case we can update
the response type to include that new field as well. Thus, all response types fields would need to be `Option<T>`
to account for any new fields that Discord may add in the future.

If you find that a response type is missing a field, please feel free to open an issue or a pull request to add that field.

## Platforms Supported 
The platforms that are supported will be the major ones as listed here:
- Linux
- MacOS
- Windows

## License
`sdkcord` is dual-licensed under [MIT](https://github.com/reaovyd/sdkcord/blob/main/LICENSE-MIT) or [Apache License Version 2.0](https://github.com/reaovyd/sdkcord/blob/main/LICENSE-APACHE)
