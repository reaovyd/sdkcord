use anyhow::Result;
use sdkcord::{
    client::spawn_client,
    config::Config,
    payload::{AuthorizeArgs, common::oauth2::OAuth2Scope},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let client = spawn_client(Config::default()).await?;
    let client = client.connect("1276759902551015485").await?;
    let response = client
        .authorize(
            AuthorizeArgs::builder()
                .client_id("1276759902551015485")
                .scopes([
                    OAuth2Scope::Rpc,
                    OAuth2Scope::Identify,
                    OAuth2Scope::Guilds,
                    OAuth2Scope::MessagesRead,
                    OAuth2Scope::RpcNotificationsRead,
                ])
                .build(),
        )
        .await?;
    info!("Authorize response: {:?}", response);
    Ok(())
}
