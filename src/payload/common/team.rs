use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::EnumString;
use thiserror::Error;

use super::user::User;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Team {
    pub icon: Option<String>,
    pub id: Option<String>,
    pub members: Option<Vec<TeamMember>>,
    pub name: Option<String>,
    pub owner_user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TeamMember {
    pub membership_state: Option<MembershipState>,
    pub team_id: Option<String>,
    pub user: Option<User>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash, EnumString)]
#[repr(u8)]
pub enum MembershipState {
    Invited = 1,
    Accepted = 2,
}

impl TryFrom<u8> for MembershipState {
    type Error = TeamError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MembershipState::Invited),
            2 => Ok(MembershipState::Accepted),
            _ => Err(TeamError::InvalidMembershipState(value)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum TeamError {
    #[error("MembershipState {0} does not exist...")]
    InvalidMembershipState(u8),
}
