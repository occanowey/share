use core::fmt;

use bytemuck::{Pod, Zeroable};
use thiserror::Error;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct RawBool<const TRUE_VALUE: u8 = 0x01, const FALSE_VALUE: u8 = 0x00> {
    pub value: u8,
}

#[derive(Debug, Error)]
pub enum ToBoolError {
    #[error("unknown value: 0x{0:02x}, should be 0x{1:02x} or 0x{2:02x}")]
    UnknownValue(u8, u8, u8),
}

impl<const TRUE_VALUE: u8, const FALSE_VALUE: u8> TryFrom<&RawBool<TRUE_VALUE, FALSE_VALUE>>
    for bool
{
    type Error = ToBoolError;

    fn try_from(value: &RawBool<TRUE_VALUE, FALSE_VALUE>) -> Result<Self, Self::Error> {
        let value = value.value;

        if value == TRUE_VALUE {
            return Ok(true);
        }

        if value == FALSE_VALUE {
            return Ok(false);
        }

        Err(ToBoolError::UnknownValue(value, TRUE_VALUE, FALSE_VALUE))
    }
}

impl<const TRUE_VALUE: u8, const FALSE_VALUE: u8> fmt::Debug for RawBool<TRUE_VALUE, FALSE_VALUE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: Result<bool, _> = self.try_into();
        match value {
            Ok(value) => {
                if f.alternate() {
                    write!(f, "SaveBool({}<0x{:02x}>)", &value, &self.value)
                } else {
                    write!(f, "{}<0x{:02x}>", &value, &self.value)
                }
            }

            Err(_) => write!(f, "SaveBool(invalid! 0x{:02x})", self.value),
        }
    }
}
