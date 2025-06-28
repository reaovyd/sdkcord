use anyhow::Result;
use sdkcord::{
    client::SdkClient,
    config::{Config, OAuth2Config},
    payload::{
        VoiceStateCreateArgs, VoiceStateDeleteArgs, VoiceStateUpdateArgs,
        common::{channel::ChannelId, oauth2::OAuth2Scope},
    },
};

const CLIENT_ID: &str = "<YOUR_CLIENT_ID_HERE>";
const CLIENT_SECRET: &str = "<YOUR_CLIENT_SECRET>";

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
    let client_listener = client.clone();
    let joiner = tokio::spawn(async move {
        loop {
            let evt = client_listener.read_event_queue().await;
            tracing::info!("Received event: {:?}", evt);
        }
    });
    let channel_id = ChannelId::from("<YOUR_CHANNEL_ID_HERE>");
    let evt = client
        .subscribe(VoiceStateCreateArgs(channel_id.clone()))
        .await;
    tracing::info!("Subscribed to event: {:?}", evt);
    let evt = client
        .subscribe(VoiceStateUpdateArgs(channel_id.clone()))
        .await;
    tracing::info!("Subscribed to event: {:?}", evt);
    let evt = client.subscribe(VoiceStateDeleteArgs(channel_id)).await;
    tracing::info!("Subscribed to event: {:?}", evt);
    joiner.await.unwrap();
    Ok(())
}
