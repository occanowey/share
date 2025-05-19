use std::{
    io::{self, Read},
    ops::Sub,
};

use crc::Crc;
use generic_array::{
    ArrayLength, GenericArray,
    typenum::{Diff, U4, U1024},
};
use tracing::{debug, trace};

pub struct ChunkReader<Inner: Sized, ChunkLength: ArrayLength + Sub<U4>>
where
    <ChunkLength as Sub<U4>>::Output: ArrayLength,
{
    inner: Inner,

    read_buffer: GenericArray<u8, ChunkLength>,
    crc: Crc<u32>,

    position: usize,
    length: usize,
    buffer: GenericArray<u8, Diff<ChunkLength, U4>>,
}

impl<Inner: Read> ChunkReader<Inner, U1024> {
    pub fn new(inner: Inner) -> Self {
        ChunkReader {
            inner,

            read_buffer: GenericArray::default(),
            crc: Crc::<u32>::new(&crc::CRC_32_MPEG_2),

            position: 0,
            length: 0,
            buffer: GenericArray::default(),
        }
    }

    pub fn into_inner(self) -> Inner {
        self.inner
    }
}

impl<Inner: Read, ChunkLength: ArrayLength + Sub<U4>> Read for ChunkReader<Inner, ChunkLength>
where
    <ChunkLength as Sub<U4>>::Output: ArrayLength,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        trace!(self.position, self.length, "read");

        // fill buffer if empty
        if self.position >= self.length {
            debug!("refillng buffer");

            // read until we fill the read buffer or reach eof
            // same as read_exact but we're fine with a partial buffer at the end of the file
            let mut partial_read_buffer = &mut self.read_buffer[..];
            let mut total_read_len = 0;

            while !partial_read_buffer.is_empty() {
                let read_len = self.inner.read(partial_read_buffer)?;

                total_read_len += read_len;
                if read_len == 0 {
                    break;
                }

                partial_read_buffer = &mut partial_read_buffer[read_len..];
            }

            trace!("read {} bytes", total_read_len);

            // pass on eof
            if total_read_len == 0 {
                return Ok(0);
            }

            // handle too few bytes
            if total_read_len < 4 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "chunk recieved less than the 4 required bytes",
                ));
            }

            let data_length = total_read_len - 4;

            // check checksum
            let checksum = self.crc.checksum(&self.read_buffer[..data_length]);
            trace!("checksum: {:08x}", checksum);
            if checksum.to_le_bytes() != self.read_buffer[data_length..total_read_len] {
                panic!("cs no match"); // TODO
            }

            trace!("filling buffer");
            // reset buffer and copy read data
            self.position = 0;
            self.length = data_length;
            self.buffer[..data_length].copy_from_slice(&self.read_buffer[..data_length]);
        }

        // copy remaining bytes in buffer or length of output buf to output buf & update position
        let min_length = buf.len().min(self.length - self.position);
        buf[..min_length]
            .copy_from_slice(&self.buffer[self.position..(self.position + min_length)]);
        self.position += min_length;

        Ok(min_length)
    }
}
