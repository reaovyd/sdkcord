#![cfg(unix)]
use common::RawDiscordIpcClient;
use reqwest::Client;
use sdkcord::payload::{
    AuthenticateArgs, AuthorizeArgs, MessageCreateArgs, MessageDeleteArgs, MessageUpdateArgs,
    PayloadRequest,
    common::{channel::ChannelId, oauth2::OAuth2Scope},
};
use serde_json::Value;

async fn subscribe_to_message_by_channel(client: &mut RawDiscordIpcClient, channel_id: &str) {
    client
        .send(
            &PayloadRequest::builder()
                .event()
                .subscribe(MessageCreateArgs(
                    ChannelId::builder().channel_id(channel_id).build(),
                ))
                .build(),
        )
        .await
        .unwrap();

    client
        .send(
            &PayloadRequest::builder()
                .event()
                .subscribe(MessageUpdateArgs(
                    ChannelId::builder().channel_id(channel_id).build(),
                ))
                .build(),
        )
        .await
        .unwrap();

    client
        .send(
            &PayloadRequest::builder()
                .event()
                .subscribe(MessageDeleteArgs(
                    ChannelId::builder().channel_id(channel_id).build(),
                ))
                .build(),
        )
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let mut client = RawDiscordIpcClient::connect("1276759902551015485").await;
    let http_client = Client::new();

    client
        .send(
            &PayloadRequest::builder()
                .request(
                    AuthorizeArgs::builder()
                        .scopes([
                            OAuth2Scope::Rpc,
                            OAuth2Scope::Identify,
                            OAuth2Scope::Guilds,
                            OAuth2Scope::MessagesRead,
                            OAuth2Scope::RpcNotificationsRead,
                        ])
                        .client_id("1276759902551015485")
                        .build(),
                )
                .build(),
        )
        .await
        .unwrap();
    let resp = client.recv().await;

    println!("{}", resp);
    let data = resp.get("data").unwrap();
    let code = data.get("code").unwrap().as_str().unwrap();

    let access_token: Value = http_client
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", "1276759902551015485"),
            ("client_secret", "CddhC2Jn6Mhr-EtogoiQAI-bOWP8D1rz"),
            ("grant_type", "authorization_code"),
            ("code", code),
        ])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{}", access_token);
    let access_token = access_token.get("access_token").unwrap().as_str().unwrap();
    println!("{}", access_token);

    client
        .send(
            &PayloadRequest::builder()
                .request(
                    AuthenticateArgs::builder()
                        .access_token(access_token)
                        .build(),
                )
                .build(),
        )
        .await
        .unwrap();

    let resp = client.recv().await;

    println!("{}", resp);

    {
        subscribe_to_message_by_channel(&mut client, "354323960722227202").await;
        subscribe_to_message_by_channel(&mut client, "401589031466434570").await;
        subscribe_to_message_by_channel(&mut client, "158748638137286656").await;
        subscribe_to_message_by_channel(&mut client, "404480479908200448").await;
        subscribe_to_message_by_channel(&mut client, "426225791169462272").await;
        subscribe_to_message_by_channel(&mut client, "457996947341443078").await;
        subscribe_to_message_by_channel(&mut client, "1203015584309051453").await;

        loop {
            let sub_payload = client.recv().await;
            println!("{}", sub_payload);
        }
    }
}

mod common;
