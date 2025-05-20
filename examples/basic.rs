use anyhow::Result;
use sdkcord::{
    client::SdkClient,
    config::{Config, OAuth2Config},
    payload::{
        GetChannelArgs,
        common::{channel::ChannelId, oauth2::OAuth2Scope},
    },
};

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
        "<YOUR_CLIENT_ID_HERE>",
        Some(
            OAuth2Config::builder()
                .client_secret("<YOUR_SECRET_HERE>")
                .scopes(scopes)
                .build(),
        ),
    )
    .await?;
    let get_channel_data = client
        .get_channel(GetChannelArgs(ChannelId::from("<YOUR_CHANNEL_ID_HERE>")))
        .await?;
    println!("Data: {:?}", get_channel_data);
    Ok(())
}
