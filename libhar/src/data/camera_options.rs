use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use super::FromRawError;

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(transparent)]
pub struct RawCameraOptions {
    value: u8,
}

#[derive(Debug)]
#[repr(u8)]
pub enum CameraMode {
    NearFollow = 2,
    FarFollow = 3,
    Bumper = 4,

    Unknown(u8),
}

#[derive(Debug)]
pub struct CameraOptions {
    pub jump_cameras_enabled: bool,
    pub inverted_camera_enabled: bool,
    pub camera_mode: CameraMode,
}

// not a huge fan of this
#[derive(Debug, Error)]
#[error("")]
pub struct FromRawCameraOptionsError;

impl FromRawError for FromRawCameraOptionsError {}

impl TryFrom<&RawCameraOptions> for CameraOptions {
    type Error = FromRawCameraOptionsError;

    fn try_from(value: &RawCameraOptions) -> Result<Self, Self::Error> {
        let RawCameraOptions { value } = *value;

        let jump_cameras_enabled = (value & 1) > 0;
        let inverted_camera_enabled = (value & 2) > 0;

        let camera_mode = match value >> 2 {
            2 => CameraMode::NearFollow,
            3 => CameraMode::FarFollow,
            4 => CameraMode::Bumper,

            0b0100_0000..=0xff => unreachable!("out of range after shift"),
            other => CameraMode::Unknown(other),
        };

        Ok(CameraOptions {
            jump_cameras_enabled,
            inverted_camera_enabled,
            camera_mode,
        })
    }
}
