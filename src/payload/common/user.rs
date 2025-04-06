use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::EnumString;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct User {
    pub accent_color: Option<u32>,
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecoration>,
    pub banner: Option<String>,
    pub banner_color: Option<String>,
    pub clan: Option<String>,
    pub discriminator: Option<String>,
    pub flags: Option<UserFlags>,
    pub global_name: Option<String>,
    pub id: Option<String>,
    pub primary_guild: Option<String>,
    pub public_flags: Option<UserFlags>,
    pub username: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub premium_type: Option<PremiumType>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AvatarDecoration {
    pub asset: Option<String>,
    pub sku: Option<String>,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash, EnumString)]
#[repr(u8)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3,
}

impl TryFrom<u8> for PremiumType {
    type Error = UserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PremiumType::None),
            1 => Ok(PremiumType::NitroClassic),
            2 => Ok(PremiumType::Nitro),
            3 => Ok(PremiumType::NitroBasic),
            _ => Err(UserError::InvalidPremiumType(value as u32)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UserFlags(u32);

bitflags! {
    impl UserFlags: u32 {
        const STAFF = 1 << 0;
        const PARTNER = 1 << 1;
        const HYPESQUAD = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        const TEAM_PSEUDO_USER = 1 << 10;
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_DEVELOPER = 1 << 17;
        const CERTIFIED_MODERATOR = 1 << 18;
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        const ACTIVE_DEVELOPER = 1 << 22;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum UserError {
    #[error("PremiumType {0} does not exist...")]
    InvalidPremiumType(u32),
}
