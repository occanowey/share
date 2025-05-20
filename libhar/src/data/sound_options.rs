use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use super::{FromRawError, RawBool, ToBoolError};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct RawSoundOptions {
    music_volume: f32,
    effects_volume: f32,
    vehicle_volume: f32,
    voice_volume: f32, // + 100.0 if mono

    is_surround: RawBool,

    _padding: [u8; 3],
}

#[derive(Debug)]
pub enum SoundMode {
    Mono,
    Stereo,
    Surround,
}

#[derive(Debug)]
pub struct SoundOptions {
    pub music_volume: f32,
    pub effects_volume: f32,
    pub vehicle_volume: f32,
    pub voice_volume: f32,

    pub sound_mode: SoundMode,
}

#[derive(Debug, Error)]
pub enum FromRawSoundOptionsError {
    #[error("error reading is_surround")]
    SurroundError(ToBoolError),

    #[error("unknown sound mode")]
    UnknownSoundMode,
}

impl FromRawError for FromRawSoundOptionsError {}

impl TryFrom<&RawSoundOptions> for SoundOptions {
    type Error = FromRawSoundOptionsError;

    fn try_from(value: &RawSoundOptions) -> Result<Self, Self::Error> {
        let is_surround =
            bool::try_from(&value.is_surround).map_err(FromRawSoundOptionsError::SurroundError)?;

        let (sound_mode, voice_volume) = match (is_surround, value.voice_volume >= 100.0) {
            (false, true) => (SoundMode::Mono, value.voice_volume - 100.0),
            (false, false) => (SoundMode::Stereo, value.voice_volume),
            (true, false) => (SoundMode::Surround, value.voice_volume),
            _ => return Err(FromRawSoundOptionsError::UnknownSoundMode),
        };

        Ok(SoundOptions {
            music_volume: value.music_volume,
            effects_volume: value.effects_volume,
            vehicle_volume: value.vehicle_volume,
            voice_volume,
            sound_mode,
        })
    }
}
