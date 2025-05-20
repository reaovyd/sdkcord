//! Creating a custom activity
//!
//! Client Secret and consequently OAuth2 isn't actually required for the SetActivity command
use std::time::Duration;

use anyhow::Result;
use sdkcord::{
    client::SdkClient,
    config::Config,
    payload::{
        SetActivityArgs,
        common::activity::{Activity, ActivityType, Assets, Party},
    },
};
use tokio::time::sleep;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let client = SdkClient::new(Config::default(), "<YOUR_CLIENT_ID>", None).await?;
    let activity = client
        .set_activity(
            SetActivityArgs::builder()
                .pid(std::process::id())
                .activity(
                    Activity::request_builder()
                        .activity_type(ActivityType::Playing)
                        .details("osaka")
                        .assets(
                            Assets::builder()
                                .large_text("osaka")
                                .small_text("osaka")
                                .large_image("yoru")
                                .small_image("yoru")
                                .build(),
                        )
                        .state("hiii")
                        .party(Party::builder().size([12, 24]).build())
                        .call(),
                )
                .build(),
        )
        .await;
    info!("{:?}", activity);
    // infinite loop so activity can remain showing on discord
    loop {
        sleep(Duration::from_secs(60 * 60)).await
    }
}
