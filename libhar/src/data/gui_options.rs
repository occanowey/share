use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use super::{FromRawError, RawBool, ToBoolError};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct RawGuiOptions {
    radar_enabled: RawBool,
}

#[derive(Debug)]
pub struct GuiOptions {
    // bool is maybe a little questionable seeing as the options are actually 'Auto' and 'Off'
    // buuuuuuuuut is's close enough
    pub radar_enabled: bool,
}

#[derive(Debug, Error)]
pub enum FromRawGuiOptionsError {
    #[error("error reading radar")]
    RadarError(ToBoolError),
}

impl FromRawError for FromRawGuiOptionsError {}

impl TryFrom<&RawGuiOptions> for GuiOptions {
    type Error = FromRawGuiOptionsError;

    fn try_from(value: &RawGuiOptions) -> Result<Self, Self::Error> {
        Ok(GuiOptions {
            radar_enabled: bool::try_from(&value.radar_enabled)
                .map_err(FromRawGuiOptionsError::RadarError)?,
        })
    }
}
