use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use super::FromRawError;

#[derive(Debug, Pod, Clone, Copy, Zeroable)]
#[repr(C, packed(1))]
pub struct RawInputOptions {
    rumble_enabled: u8,
}

#[derive(Debug)]
pub struct InputOptions {
    pub rumble_enabled: bool,
}

#[derive(Debug, Error)]
pub enum FromRawInputOptionsError {
    #[error("unknown rumble value: {0}, should be 0xff or 0x00")]
    UnknownRumbleValue(u8),
}

impl FromRawError for FromRawInputOptionsError {}

impl TryFrom<&RawInputOptions> for InputOptions {
    type Error = FromRawInputOptionsError;

    fn try_from(value: &RawInputOptions) -> Result<Self, Self::Error> {
        let rumble_enabled = match value.rumble_enabled {
            0xff => true,
            0x00 => false,
            other => return Err(FromRawInputOptionsError::UnknownRumbleValue(other)),
        };

        Ok(InputOptions { rumble_enabled })
    }
}
