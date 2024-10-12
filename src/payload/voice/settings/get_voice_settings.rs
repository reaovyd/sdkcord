use crate::payload::macros::make_command_reqres_payload;

make_command_reqres_payload!(
    GetVoiceSettings,
    (
        /// Used to retrieve the client's voice settings
    )
);
