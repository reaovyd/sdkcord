use crate::payload::macros::make_subscription_event;

make_subscription_event!(ActivityJoin,
    (
        /// sent when the user clicks a Rich Presence join invite in chat to join a game
    )
);

make_subscription_event!(ActivitySpectate,
    (#[doc = "sent when the user clicks a Rich Presence spectate invite in chat to spectate a game"])
);

make_subscription_event!(ActivityJoinRequest,
    (#[doc = "sent when the user receives a Rich Presence Ask to Join request"])
);
