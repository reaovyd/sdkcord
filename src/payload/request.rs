use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    Command, Event, Payload, {ArgsType, EventArgsType, RequestArgsType},
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PayloadRequest(Box<Payload>);

impl PayloadRequest {
    #[inline(always)]
    pub const fn builder() -> PayloadRequestBuilder<EmptyArgs, EmptyRType> {
        PayloadRequestBuilder { args: None, evt: None, cmd: None, _rtype: PhantomData }
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
    #[inline(always)]
    pub fn request<A: RequestArgsType>(self, args: A) -> PayloadRequestBuilder<A, WithRequest> {
        let cmd = args.name();
        PayloadRequestBuilder { args: Some(args), evt: None, cmd: Some(cmd), _rtype: PhantomData }
    }

    #[inline(always)]
    pub const fn event<A: EventArgsType>(self) -> PayloadRequestBuilder<A, WithEvent> {
        PayloadRequestBuilder { args: None, evt: None, cmd: None, _rtype: PhantomData }
    }
}

impl<A: EventArgsType> PayloadRequestBuilder<A, WithEvent> {
    #[inline(always)]
    pub fn subscribe(self, args: A) -> PayloadRequestBuilder<A, WithEvent> {
        let evt = args.name();
        PayloadRequestBuilder {
            args: Some(args),
            evt: Some(evt),
            cmd: Some(Command::Subscribe),
            _rtype: PhantomData,
        }
    }

    #[inline(always)]
    pub fn unsubscribe(self, args: A) -> PayloadRequestBuilder<A, WithEvent> {
        let evt = args.name();
        PayloadRequestBuilder {
            args: Some(args),
            evt: Some(evt),
            cmd: Some(Command::Unsubscribe),
            _rtype: PhantomData,
        }
    }
}

impl<A: ArgsType, RType> PayloadRequestBuilder<A, RType> {
    #[inline(always)]
    pub fn build(self) -> PayloadRequest {
        let cmd = self.cmd.unwrap();
        let evt = self.evt;
        let args = self.args.unwrap().args_val();

        let payload = Payload { cmd, nonce: Uuid::new_v4(), evt, data: None, args: Some(args) };
        PayloadRequest(Box::new(payload))
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
pub struct WithEvent;

#[cfg(test)]
mod tests {
    use crate::payload::{
        common::{
            activity::{Activity, ActivityType, Party},
            channel::ChannelId,
            oauth2::OAuth2Scope,
        },
        AuthenticateArgs, AuthorizeArgs, GetChannelArgs, GetGuildArgs, GetGuildsArgs,
        GuildStatusArgs, SetActivityArgs,
    };

    use super::PayloadRequest;

    #[test]
    fn construct_args_payload_authorize() {
        let request = PayloadRequest::builder()
            .request(
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
            .request(AuthenticateArgs::builder().access_token("access_token1").build())
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""access_token":"access_token1""#));
    }

    #[test]
    fn construct_args_payload_get_guild() {
        let request = PayloadRequest::builder()
            .request(GetGuildArgs::builder().guild_id("guild_id12").build())
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#""guild_id":"guild_id12""#));
        assert!(!request.contains("timeout"));
    }

    #[test]
    fn construct_args_payload_get_guilds() {
        let request = PayloadRequest::builder().request(GetGuildsArgs::default()).build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#"args":{}"#));
    }

    #[test]
    fn construct_args_payload_get_channel() {
        let request = PayloadRequest::builder()
            .request(GetChannelArgs(ChannelId::builder().channel_id("123").build()))
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#"channel_id":"123""#));
    }

    #[test]
    fn construct_args_payload_event_guild_status() {
        let request = PayloadRequest::builder()
            .event()
            .subscribe(GuildStatusArgs::builder().guild_id("123").build())
            .build();
        let request = serde_json::to_string(&request).unwrap();
        assert!(request.contains(r#"guild_id":"123""#));
        assert!(request.contains(r#"evt":"GUILD_STATUS""#));
    }

    #[test]
    fn construct_args_payload_set_activity() {
        let request = PayloadRequest::builder()
            .request(
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
