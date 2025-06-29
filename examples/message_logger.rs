use anyhow::Result;
use sdkcord::{
    client::SdkClient,
    config::{Config, OAuth2Config},
    payload::{
        MessageCreateArgs, MessageDeleteArgs, MessageUpdateArgs,
        common::{channel::ChannelId, oauth2::OAuth2Scope},
    },
};
use tracing::info;

const CLIENT_ID: &str = "<YOUR_CLIENT_ID>";
const CLIENT_SECRET: &str = "<YOUR_CLIENT_SECRET>";

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
    let channels = [
        "<YOUR_CHANNEL_ID>",
        "<YOUR_CHANNEL_ID>",
        "<YOUR_CHANNEL_ID>",
        "<YOUR_CHANNEL_ID>",
        "<YOUR_CHANNEL_ID>",
        "<YOUR_CHANNEL_ID>",
        "<YOUR_CHANNEL_ID>",
    ];
    for channel in channels {
        let channel_id = ChannelId::from(channel);
        info!("Subscribing to channel: {}", channel_id.channel_id);
        subscribe_to_channel(&client, channel_id).await?;
    }
    loop {
        let data = client.read_event_queue().await;
        match data {
            sdkcord::payload::EventData::MessageCreate(message_create_data) => {
                info!("MESSAGE CREATED: {:?}", message_create_data);
            }
            sdkcord::payload::EventData::MessageUpdate(message_update_data) => {
                info!("MESSAGE UPDATED: {:?}", message_update_data);
            }
            sdkcord::payload::EventData::MessageDelete(message_delete_data) => {
                info!("MESSAGE DELETED: {:?}", message_delete_data);
            }
            _ => {}
        };
    }
}
