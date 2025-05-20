use bytemuck::{Pod, Zeroable};
use chrono::{NaiveDate, NaiveDateTime};
use thiserror::Error;

use super::FromRawError;

const SAVE_MAGIC_VALUE: u16 = 1978;

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct RawSaveInfo {
    magic: u16,
    save_time: RawDateTime,
    current_level: u8,
    current_mission: u8,
    save_size: u32,
}

#[derive(Debug)]
pub struct SaveInfo {
    pub save_time: NaiveDateTime,
    pub save_size: usize,

    // TODO: use enums for these
    pub current_level: u8,
    pub current_mission: u8,
}

#[derive(Debug, Error)]
pub enum FromRawSaveInfoError {
    #[error("incorrect magic value, expected: 1978, got: {0}")]
    BadMagic(u16),

    #[error("invalid save time")]
    SaveTimeError(#[from] FromRawDateTimeError),
}

impl FromRawError for FromRawSaveInfoError {}

impl TryFrom<&RawSaveInfo> for SaveInfo {
    type Error = FromRawSaveInfoError;

    fn try_from(value: &RawSaveInfo) -> Result<Self, Self::Error> {
        if value.magic != SAVE_MAGIC_VALUE {
            return Err(FromRawSaveInfoError::BadMagic(value.magic));
        }

        // todo check mission & level are valid

        Ok(SaveInfo {
            save_time: (&value.save_time).try_into()?,
            save_size: value.save_size as usize,

            current_level: value.current_level,
            current_mission: value.current_mission,
        })
    }
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed(1))]
struct RawDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,

    _padding: u8,
}

#[derive(Debug, Error)]
pub enum FromRawDateTimeError {
    #[error("{0}-{1}-{2} is not a real date")]
    InvalidYMD(i32, u32, u32),

    #[error("{0}-{1}-{2} is not a real time")]
    InvalidHMS(u32, u32, u32),
}

impl TryFrom<&RawDateTime> for NaiveDateTime {
    type Error = FromRawDateTimeError;

    fn try_from(value: &RawDateTime) -> Result<Self, Self::Error> {
        let RawDateTime {
            year: y,
            month: mo,
            day: d,
            hour: h,
            minute: mi,
            second: s,
            ..
        } = *value;

        NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
            .ok_or_else(|| FromRawDateTimeError::InvalidYMD(y as i32, mo as u32, d as u32))?
            .and_hms_opt(h as u32, mi as u32, s as u32)
            .ok_or_else(|| FromRawDateTimeError::InvalidHMS(h as u32, mi as u32, s as u32))
    }
}
