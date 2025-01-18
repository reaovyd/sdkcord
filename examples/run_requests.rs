use common::RawDiscordIpcClient;
use ipccord::payload::{
    common::{channel::ChannelId, oauth2::OAuth2Scope},
    reqres::{
        AuthenticateArgs, AuthorizeArgs, MessageCreateArgs, MessageDeleteArgs, MessageUpdateArgs,
    },
    request_builder::PayloadRequest,
};
use reqwest::Client;
use serde_json::Value;

async fn subscribe_to_message_by_channel(client: &mut RawDiscordIpcClient, channel_id: &str) {
    client
        .send(
            &PayloadRequest::builder()
                .event()
                .subscribe(MessageCreateArgs(ChannelId::builder().channel_id(channel_id).build()))
                .build(),
        )
        .await
        .unwrap();

    client
        .send(
            &PayloadRequest::builder()
                .event()
                .subscribe(MessageUpdateArgs(ChannelId::builder().channel_id(channel_id).build()))
                .build(),
        )
        .await
        .unwrap();

    client
        .send(
            &PayloadRequest::builder()
                .event()
                .subscribe(MessageDeleteArgs(ChannelId::builder().channel_id(channel_id).build()))
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
            ("client_secret", "2nygKeJU085KoLPLr44_FF7ZZPLPprXj"),
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
                .request(AuthenticateArgs::builder().access_token(access_token).build())
                .build(),
        )
        .await
        .unwrap();

    let resp = client.recv().await;

    println!("{}", resp);

    // client
    //     .send(
    //         &PayloadRequest::builder()
    //             .request()
    //
    // .args(GetChannelArgs(ChannelId::builder().channel_id("354323960722227202").
    // build()))             .build(),
    //     )
    //     .await
    //     .unwrap();
    // let resp = client.recv().await;
    // println!("{}", resp);

    {
        subscribe_to_message_by_channel(&mut client, "354323960722227202").await;
        subscribe_to_message_by_channel(&mut client, "401589031466434570").await;
        subscribe_to_message_by_channel(&mut client, "158748638137286656").await;
        subscribe_to_message_by_channel(&mut client, "404480479908200448").await;
        subscribe_to_message_by_channel(&mut client, "426225791169462272").await;
        subscribe_to_message_by_channel(&mut client, "457996947341443078").await;

        loop {
            let sub_payload = client.recv().await;
            println!("{}", sub_payload);
        }
    }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //
    // .args(GetGuildArgs::builder().guild_id("478925420616482816").build())
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(&PayloadRequest::builder().request().
    // args(GetGuildsArgs::default()).build())         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //                 .args(GetChannelArgs(
    //
    // ChannelId::builder().channel_id("354323960722227202").build(),
    //                 ))
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //                 .args(GetChannelsArgs(
    //
    // GuildId::builder().guild_id("158748638137286656").build(),
    //                 ))
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //
    // .args(SetUserVoiceSettingsArgs::builder().user_id("486308512671203348").
    // build())                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // // {
    // //     let resp = client
    // //         .send(
    // //             &PayloadRequest::builder()
    // //                 .request()
    // //                 .args(
    // //
    // SelectVoiceChannelArgs::builder().channel_id("572225910485286912").
    // build(), //                 )
    // //                 .build(),
    // //         )
    // //         .await;
    // //     println!("{}", resp);
    // // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //                 .args(GetSelectedVoiceChannelArgs::default())
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //
    // .args(SelectTextChannelArgs::builder().channel_id("354323960722227202").
    // build())                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //
    // &PayloadRequest::builder().request().
    // args(GetVoiceSettingsArgs::default()).build(),         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //
    // .args(SetVoiceSettingsArgs(VoiceSettings::builder().build()))
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //                 .args(
    //                     SetActivityArgs::builder()
    //                         .pid(std::process::id())
    //                         .activity(
    //                             Activity::request_builder()
    //                                 .activity_type(ActivityType::Playing)
    //                                 .state("fuhnite")
    //                                 .details("fornit")
    //                                 .timestamps(
    //                                     Timestamps::builder()
    //                                         .start(
    //                                             SystemTime::now()
    //
    // .duration_since(SystemTime::UNIX_EPOCH)
    // .unwrap()                                                 .as_secs(),
    //                                         )
    //                                         .build(),
    //                                 )
    //                                 .party(Party::builder().size([1,
    // 4]).build())                                 .assets(
    //                                     Assets::builder()
    //                                         .large_image("rp_gaming")
    //                                         .large_text("rap gam")
    //                                         .small_image("rp_gaming")
    //                                         .small_text("rap gam")
    //                                         .build(),
    //                                 )
    //                                 .secrets(
    //                                     Secrets::builder()
    //                                         .join("hii")
    //                                         .spectate("hii2")
    //                                         .secrets_match("hii3")
    //                                         .build(),
    //                                 )
    //                                 .call(),
    //                         )
    //                         .build(),
    //                 )
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //
    // .args(SetActivityArgs::builder().pid(std::process::id()).build())
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // {
    //     let resp = client
    //         .send(
    //             &PayloadRequest::builder()
    //                 .request()
    //                 .args(
    //
    // SendActivityJoinInviteArgs::builder().user_id("158284148040138752").
    // build(),                 )
    //                 .build(),
    //         )
    //         .await;
    //     println!("{}", resp);
    // }

    // let resp = client
    //     .send(
    //         &PayloadRequest::builder()
    //             .request()
    //             .args(GetSelectedVoiceChannelArgs::default())
    //             .build(),
    //     )
    //     .await;
    // println!("{}", resp);

    // let resp = client
    //     .send(
    //         &PayloadRequest::builder()
    //             .request()
    //             .args(GetSelectedVoiceChannelArgs::default())
    //             .build(),
    //     )
    //     .await;
    // println!("{}", resp);

    // let resp = client
    //     .send(
    //         &PayloadRequest::builder()
    //             .request()
    //
    // .args(AuthenticateArgs::builder().access_token(access_token).build())
    //             .build(),
    //     )
    //     .await;
    // println!("{}", resp);

    // let resp = client
    //     .send(
    //         &PayloadRequest::builder()
    //             .request()
    //
    // .args(GetGuildArgs::builder().guild_id("158748638137286656").build())
    //             .build(),
    //     )
    //     .await;
    // println!("{}", resp);

    // let resp = client
    //     .send(&PayloadRequest::builder().request().
    // args(GetGuildsArgs::default()).build())     .await;
    // println!("{}", resp);
}

mod common;
