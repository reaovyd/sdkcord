use anyhow::Result;
use sdkcord::{
    client::SdkClient,
    config::{Config, OAuth2Config},
    payload::{
        MessageCreateArgs, MessageDeleteArgs, MessageUpdateArgs,
        common::{channel::ChannelId, oauth2::OAuth2Scope},
    },
};

const CLIENT_ID: &str = "<YOUR_CLIENT_ID>";
const CLIENT_SECRET: &str = "<YOUR_CLIENT_SECRET>";
const CHANNEL_ID: &str = "<YOUR_CHANNEL_ID>";

async fn subscribe_to_channel(client: &SdkClient, channel_id: ChannelId) -> Result<()> {
    client
        .subscribe(MessageCreateArgs(channel_id.clone()))
        .await?;
    client
        .subscribe(MessageDeleteArgs(channel_id.clone()))
        .await?;
    client.subscribe(MessageUpdateArgs(channel_id)).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let scopes = [
        OAuth2Scope::Rpc,
        OAuth2Scope::Identify,
        OAuth2Scope::Guilds,
        OAuth2Scope::MessagesRead,
        OAuth2Scope::RpcNotificationsRead,
    ];
    let client = SdkClient::new(
        Config::default(),
        CLIENT_ID,
        Some(
            OAuth2Config::builder()
                .client_secret(CLIENT_SECRET)
                .scopes(scopes)
                .build(),
        ),
    )
    .await?;
    subscribe_to_channel(&client, ChannelId::from(CHANNEL_ID)).await?;
    loop {
        let data = client.read_event_queue().await;
        println!("Received event: {:?}", data);
    }
}
