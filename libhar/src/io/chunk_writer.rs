use std::{
    io::{self, Write},
    ops::Sub,
};

use crc::Crc;
use generic_array::{
    ArrayLength, GenericArray,
    typenum::{Diff, U4, U1024},
};
use tracing::trace;

pub struct ChunkWriter<Inner: Sized, ChunkLength: ArrayLength + Sub<U4>>
where
    <ChunkLength as Sub<U4>>::Output: ArrayLength,
{
    inner: Inner,

    write_buffer: GenericArray<u8, ChunkLength>,
    crc: Crc<u32>,

    position: usize,
    buffer: GenericArray<u8, Diff<ChunkLength, U4>>,
}

impl<Inner: Write> ChunkWriter<Inner, U1024> {
    pub fn new(inner: Inner) -> Self {
        ChunkWriter {
            inner,

            write_buffer: GenericArray::default(),
            crc: Crc::<u32>::new(&crc::CRC_32_MPEG_2),

            position: 0,
            buffer: GenericArray::default(),
        }
    }

    pub fn into_inner(self) -> Inner {
        self.inner
    }
}

impl<Inner: Write, ChunkLength: ArrayLength + Sub<U4>> ChunkWriter<Inner, ChunkLength>
where
    <ChunkLength as Sub<U4>>::Output: ArrayLength,
{
    fn write_chunk(&mut self) -> io::Result<()> {
        trace!("writing {} bytes to inner writer", self.position + 4);

        // write data & checksum to output buffer
        let checksum = self.crc.checksum(&self.buffer[..self.position]);
        trace!("checksum: {:08x}", checksum);

        self.write_buffer[..self.position].copy_from_slice(&self.buffer[..self.position]);
        self.write_buffer[self.position..(self.position + 4)]
            .copy_from_slice(&checksum.to_le_bytes());

        // write output buffer and reset position
        self.inner
            .write_all(&self.write_buffer[..(self.position + 4)])?;
        self.position = 0;

        Ok(())
    }
}

/// flush is required after writing
///
/// calling flush between writes could cause partial (less than 1024 bytes) chunks to we wrote to inner
impl<Inner: Write, ChunkLength: ArrayLength + Sub<U4>> Write for ChunkWriter<Inner, ChunkLength>
where
    <ChunkLength as Sub<U4>>::Output: ArrayLength,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let min_length = buf.len().min(self.buffer.len() - self.position);
        trace!("copying {min_length} bytes from source buffer to working buffer");
        self.buffer[self.position..(self.position + min_length)]
            .copy_from_slice(&buf[..min_length]);
        self.position += min_length;

        if self.position >= self.buffer.len() {
            self.write_chunk()?;
        }

        Ok(min_length)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write_chunk()?;
        self.inner.flush()
    }
}
