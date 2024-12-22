use common::RawDiscordIpcClient;

#[tokio::test]
async fn test_authenticate_call() {
    let client = RawDiscordIpcClient::connect("1276759902551015485").await;
}

mod common;
