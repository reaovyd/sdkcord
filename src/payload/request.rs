use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    args::{ArgsType, EventArgsType, RequestArgsType},
    Command, Event, Payload,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct PayloadRequest(Payload);

impl PayloadRequest {
    pub fn builder() -> PayloadRequestBuilder<EmptyArgs, EmptyRType> {
        PayloadRequestBuilder::default()
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct PayloadRequestBuilder<A, RType> {
    args: Option<A>,
    evt: Option<Event>,
    cmd: Option<Command>,
    _rtype: PhantomData<RType>,
}

impl PayloadRequestBuilder<EmptyArgs, EmptyRType> {
    pub const fn request<A>(self) -> PayloadRequestBuilder<A, WithRequest> {
        PayloadRequestBuilder { args: None, evt: None, cmd: None, _rtype: PhantomData }
    }

    pub const fn subscribe<A>(self) -> PayloadRequestBuilder<A, WithSubscribe> {
        PayloadRequestBuilder {
            args: None,
            evt: None,
            cmd: Some(Command::Subscribe),
            _rtype: PhantomData,
        }
    }

    pub const fn unsubscribe<A>(self) -> PayloadRequestBuilder<A, WithUnsubscribe> {
        PayloadRequestBuilder {
            args: None,
            evt: None,
            cmd: Some(Command::Unsubscribe),
            _rtype: PhantomData,
        }
    }
}

impl<A: RequestArgsType> PayloadRequestBuilder<A, WithRequest> {
    pub fn args(self, cmd_args: A) -> PayloadRequestBuilder<A, WithRequest> {
        let cmd = cmd_args.name();
        PayloadRequestBuilder {
            args: Some(cmd_args),
            evt: self.evt,
            cmd: Some(cmd),
            _rtype: PhantomData,
        }
    }
}

impl<A: EventArgsType, RType: SubscribeRType> PayloadRequestBuilder<A, RType> {
    pub fn args(self, event_args: A) -> PayloadRequestBuilder<A, RType> {
        let evt = event_args.name();
        PayloadRequestBuilder {
            args: Some(event_args),
            evt: Some(evt),
            cmd: self.cmd,
            _rtype: PhantomData,
        }
    }
}

impl<A: ArgsType, RType> PayloadRequestBuilder<A, RType> {
    pub fn build(self) -> PayloadRequest {
        let cmd = self.cmd.unwrap();
        let evt = self.evt;
        let args = self.args.unwrap().args_val();

        let payload = Payload { cmd, nonce: Uuid::new_v4(), evt, data: None, args: Some(args) };
        PayloadRequest(payload)
    }
}

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct EmptyArgs;
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct EmptyRType;
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct NoEvent;
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct WithRequest;
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct WithSubscribe;
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct WithUnsubscribe;

mod sealed {
    pub trait Sealed {}
}

pub trait SubscribeRType: sealed::Sealed {}
impl sealed::Sealed for WithSubscribe {}
impl sealed::Sealed for WithUnsubscribe {}

impl SubscribeRType for WithUnsubscribe {}
impl SubscribeRType for WithSubscribe {}

#[cfg(test)]
mod tests {
    use crate::payload::{
        args::{
            AuthenticateArgs, AuthorizeArgs, GetChannelArgs, GetGuildArgs, GetGuildsArgs,
            GuildStatusArgs, SetActivityArgs,
        },
        types::{
            activity::{Activity, ActivityType, Party},
            channel::ChannelId,
            oauth2::OAuth2Scope,
        },
    };

    use super::PayloadRequest;

    #[test]
    fn construct_args_payload_authorize() {
        let request = PayloadRequest::builder()
            .request()
            .args(
                AuthorizeArgs::builder()
                    .scopes([
                        OAuth2Scope::Rpc,
                        OAuth2Scope::DmChannelsRead,
                        OAuth2Scope::ApplicationsCommandsPermissionsUpdate,
                    ])
                    .client_id("abcdef")
                    .rpc_token("12345")
                    .username("123")
                    .build(),
            )
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""client_id":"abcdef""#));
        assert!(request.contains(r#""rpc_token":"12345""#));
    }

    #[test]
    fn construct_args_payload_authenticate() {
        let request = PayloadRequest::builder()
            .request()
            .args(AuthenticateArgs::builder().access_token("access_token1").build())
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""access_token":"access_token1""#));
    }

    #[test]
    fn construct_args_payload_get_guild() {
        let request = PayloadRequest::builder()
            .request()
            .args(GetGuildArgs::builder().guild_id("guild_id12").build())
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""guild_id":"guild_id12""#));
        assert!(!request.contains("timeout"));
    }

    #[test]
    fn construct_args_payload_get_guilds() {
        let request = PayloadRequest::builder().request().args(GetGuildsArgs::default()).build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#"args":{}"#));
    }

    #[test]
    fn construct_args_payload_get_channel() {
        let request = PayloadRequest::builder()
            .request()
            .args(GetChannelArgs(ChannelId::builder().channel_id("123").build()))
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#"channel_id":"123""#));
    }

    #[test]
    fn construct_args_payload_event_guild_status() {
        let request = PayloadRequest::builder()
            .subscribe()
            .args(GuildStatusArgs::builder().guild_id("123").build())
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#"guild_id":"123""#));
        assert!(request.contains(r#"evt":"GUILD_STATUS""#));
    }

    #[test]
    fn construct_args_payload_set_activity() {
        let request = PayloadRequest::builder()
            .request()
            .args(
                SetActivityArgs::builder()
                    .pid(12)
                    .activity(
                        Activity::request_builder()
                            .activity_type(ActivityType::Competing)
                            .state("abcdef")
                            .party(Party::builder().size([1, 4]).build())
                            .call(),
                    )
                    .build(),
            )
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains("\"state\":\"abcdef\""));
        assert!(request.contains("\"type\":5"))
    }
}
