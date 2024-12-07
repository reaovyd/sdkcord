use std::marker::PhantomData;

use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use super::{
    args::{
        ArgsType,
        EventArgsType,
        RequestArgsType,
    },
    Command,
    Event,
    Payload,
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
