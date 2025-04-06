use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MfaLevel {
    None = 0,
    Elevated = 1,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExplicitContentFilterLevel {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NsfwLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

impl TryFrom<u8> for NsfwLevel {
    type Error = GuildError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NsfwLevel::Default),
            1 => Ok(NsfwLevel::Explicit),
            2 => Ok(NsfwLevel::Safe),
            3 => Ok(NsfwLevel::AgeRestricted),
            _ => Err(GuildError::InvalidNsfwLevel(value)),
        }
    }
}

impl TryFrom<u8> for MfaLevel {
    type Error = GuildError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MfaLevel::None),
            1 => Ok(MfaLevel::Elevated),
            _ => Err(GuildError::InvalidMfaLevel(value)),
        }
    }
}

impl TryFrom<u8> for VerificationLevel {
    type Error = GuildError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VerificationLevel::None),
            1 => Ok(VerificationLevel::Low),
            2 => Ok(VerificationLevel::Medium),
            3 => Ok(VerificationLevel::High),
            4 => Ok(VerificationLevel::VeryHigh),
            _ => Err(GuildError::InvalidVerificationLevel(value)),
        }
    }
}

impl TryFrom<u8> for MessageNotificationLevel {
    type Error = GuildError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageNotificationLevel::AllMessages),
            1 => Ok(MessageNotificationLevel::OnlyMentions),
            _ => Err(GuildError::InvalidMessageNotificationLevel(value)),
        }
    }
}

impl TryFrom<u8> for ExplicitContentFilterLevel {
    type Error = GuildError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExplicitContentFilterLevel::Disabled),
            1 => Ok(ExplicitContentFilterLevel::MembersWithoutRoles),
            2 => Ok(ExplicitContentFilterLevel::AllMembers),
            _ => Err(GuildError::InvalidExplicitContentFilterLevel(value)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum GuildError {
    #[error("Verification Level {0} does not exist...")]
    InvalidVerificationLevel(u8),
    #[error("Message notification level {0} does not exist...")]
    InvalidMessageNotificationLevel(u8),
    #[error("Explicit content filter level {0} does not exist...")]
    InvalidExplicitContentFilterLevel(u8),
    #[error("MFA level {0} does not exist...")]
    InvalidMfaLevel(u8),
    #[error("NSFW level {0} does not exist...")]
    InvalidNsfwLevel(u8),
}
