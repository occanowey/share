use std::{
    error::Error,
    fmt::Debug,
    io::{self, Read},
};

use bytemuck::{AnyBitPattern, from_bytes};
use thiserror::Error;

mod primitives;
pub use primitives::*;

mod save_info;
pub use save_info::*;

mod input_options;
pub use input_options::*;

mod sound_options;
pub use sound_options::*;

mod camera_options;
pub use camera_options::*;

mod gui_options;
pub use gui_options::*;

pub trait FromRawError: Error {}

#[derive(Debug, Error)]
pub enum ReadError<E: FromRawError> {
    #[error("io error")]
    IoError(#[from] io::Error),

    #[error("error converting from raw value")]
    FromRawError(#[from] E),
}

pub fn read_section<T, R, E>(reader: &mut impl Read) -> Result<T, ReadError<E>>
where
    T: for<'r> TryFrom<&'r R, Error = E>,
    R: AnyBitPattern,
    E: FromRawError,
{
    let mut bytes = vec![0; std::mem::size_of::<R>()];
    reader.read_exact(&mut bytes)?;

    let raw: &R = from_bytes(&bytes);
    Ok(raw.try_into()?)
}
