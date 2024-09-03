use crate::payload::request::macros::make_request_payload;
use derive_builder::Builder;
use ordered_float::OrderedFloat;
use paste::paste;
use serde::Serialize;
use serde_with::skip_serializing_none;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Pan {
    left: OrderedFloat<f32>,
    right: OrderedFloat<f32>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Volume {
    #[serde(flatten)]
    inner: u8,
}

impl Volume {
    const MAX_VOL: u8 = 200;

    /// Creates a new [`Volume`] type
    ///
    /// The [`Default`] volume is 100.
    ///
    /// # Errors
    /// As per Discord's docs [here][discorddocs], the boundaries for the
    /// `volume` type are between 0 and 200. Since u8 can never be negative,
    /// we only need to check if it's above 200.
    ///
    /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setuservoicesettings
    pub const fn new(inner: u8) -> Result<Self, Error> {
        if inner > Self::MAX_VOL {
            Err(Error::VolumeBoundary { vol: inner })
        } else {
            Ok(Self { inner })
        }
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self { inner: 100 }
    }
}

impl Pan {
    const MIN_PAN: OrderedFloat<f32> = OrderedFloat(0.0);
    const MAX_PAN: OrderedFloat<f32> = OrderedFloat(1.0);

    /// Creates a new [`Pan`] type
    ///
    /// # Errors
    /// As per Discord's docs [here][discorddocs], the boundaries for the fields
    /// in a `pan` type are between 0.0 and 1.0.
    ///
    /// If what is passed in as arguments for these parameters, then the
    /// function will return an [`enum@Error`], which would contain what you
    /// have passed in as well. See [`enum@Error`] for more.
    ///
    /// [discorddocs]: https://discord.com/developers/docs/topics/rpc#setuservoicesettings-pan-object
    pub fn new(left: f32, right: f32) -> Result<Self, Error> {
        let ord_left = OrderedFloat(left);
        let ord_right = OrderedFloat(right);
        // TODO: maybe can get rid of this NAN check anyways since according to the
        // [`ordered_float`] docs they count nan to be the highest
        if (ord_left.is_nan() || ord_right.is_nan())
            || (ord_left < Self::MIN_PAN || ord_right < Self::MIN_PAN)
            || (ord_left > Self::MAX_PAN || ord_right > Self::MAX_PAN)
        {
            Err(Error::PanBoundary { left, right })
        } else {
            Ok(Self { left: ord_left, right: ord_right })
        }
    }
}

make_request_payload!(
    SetUserVoiceSettings,
    (user_id, String, "user id"),
    (pan, Option<Pan>, "set the pan of the user"),
    (volume, Option<Volume>, "set the volume of user (defaults to 100, min 0, max 200)"),
    (mute, Option<bool>, "set the mute state of the user")
);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error setting pan; got values {left} {right}")]
    PanBoundary { left: f32, right: f32 },
    #[error("Error setting volume; got value {vol}")]
    VolumeBoundary { vol: u8 },
}
