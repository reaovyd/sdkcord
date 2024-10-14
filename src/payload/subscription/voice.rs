use crate::payload::macros::make_subscription_event;

make_subscription_event!(VoiceChannelSelect,
    (#[doc = "sent when the client joins a voice channel"])
);

make_subscription_event!(VoiceStateCreate,
    (#[doc = "sent when a user joins a subscribed voice channel"]),
    (guild_id, String, (#[doc = "id of channel to listen to updates of"]), (#[builder(into)]))
);

make_subscription_event!(VoiceStateUpdate,
    (#[doc = "sent when a user's voice state changes in a subscribed voice channel (mute, volume, etc.)"]),
    (guild_id, String, (#[doc = "id of channel to listen to updates of"]), (#[builder(into)]))
);

make_subscription_event!(VoiceStateDelete,
    (#[doc = "sent when a user parts a subscribed voice channel"]),
    (guild_id, String, (#[doc = "id of channel to listen to updates of"]), (#[builder(into)]))
);

make_subscription_event!(VoiceSettingsUpdate,
    (#[doc = "sent when the client's voice settings update"])
);

make_subscription_event!(VoiceConnectionStatus,
    (#[doc = "sent when the client's voice connection status changes"])
);
