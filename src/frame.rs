use std::io::{self, Write};
use std::ptr;

use libc::size_t;
use lz4_sys::{
    LZ4FCompressionContext, LZ4F_compressBound, LZ4F_createCompressionContext,
    LZ4F_freeCompressionContext, LZ4F_getBlockSize, LZ4F_getVersion,
};

use super::error::LZ4FError;

#[derive(Debug)]
pub struct LZ4FEncoder<W: Write> {
    inner: W,
    context: LZ4FCompressionContext,
    block_size: size_t,
    buffer: Vec<u8>,
}

impl<W: Write> LZ4FEncoder<W> {
    pub fn new(inner: W) -> io::Result<LZ4FEncoder<W>> {
        let mut context: LZ4FCompressionContext = ptr::null_mut();
        LZ4FError::check(unsafe {
            LZ4F_createCompressionContext(&mut context, LZ4F_getVersion())
        })?;

        let block_size = LZ4FError::check(unsafe { LZ4F_getBlockSize(0) })?;

        let buffer_size = LZ4FError::check(unsafe { LZ4F_compressBound(block_size, ptr::null()) })?;
        let buffer = Vec::with_capacity(buffer_size);

        Ok(LZ4FEncoder {
            inner,
            context,
            block_size,
            buffer,
        })
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W: Write> Drop for LZ4FEncoder<W> {
    fn drop(&mut self) {
        unsafe { LZ4F_freeCompressionContext(self.context) };
    }
}

impl<W: Write> Write for LZ4FEncoder<W> {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
