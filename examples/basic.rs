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
            ChannelId::builder().channel_id("some_channel_id").build(),
        ))
        .build();
    let response = client.send_request(request).await?;
    println!("{:?}", response);
    Ok(())
}
