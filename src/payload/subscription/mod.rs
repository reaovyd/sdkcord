pub use activity::*;
pub use channel::*;
pub use guild::*;
pub use message::*;
pub use notification::*;
pub use speaking::*;
pub use voice::*;

mod activity;
mod channel;
mod guild;
mod message;
mod notification;
mod speaking;
mod voice;

#[cfg(test)]
mod tests {
    use crate::payload::{
        GuildStatusEventRequest,
        GuildStatusEventRequestArgs,
    };

    #[test]
    fn test_evt_exists_subscribe() {
        let evt = GuildStatusEventRequest::new(
            GuildStatusEventRequestArgs::builder().guild_id("asdasd").build(),
        )
        .build();
        let json = serde_json::to_string(&evt).unwrap();
        assert!(json.contains(r#"{"evt":"GUILD_STATUS","#))
    }
}
