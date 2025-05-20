use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use super::{FromRawError, RawBool, ToBoolError};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct RawInputOptions {
    rumble_enabled: RawBool<0xff, 0x00>,
}

#[derive(Debug)]
pub struct InputOptions {
    pub rumble_enabled: bool,
}

#[derive(Debug, Error)]
pub enum FromRawInputOptionsError {
    #[error("error reading rumble")]
    RumbleError(ToBoolError),
}

impl FromRawError for FromRawInputOptionsError {}

impl TryFrom<&RawInputOptions> for InputOptions {
    type Error = FromRawInputOptionsError;

    fn try_from(value: &RawInputOptions) -> Result<Self, Self::Error> {
        Ok(InputOptions {
            // try_from rather than try_into to make types easier
            rumble_enabled: bool::try_from(&value.rumble_enabled)
                .map_err(FromRawInputOptionsError::RumbleError)?,
        })
    }
}
