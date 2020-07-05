use std::io::Read;

use bytes::BytesMut;
use log::*;
use nom::{Err, Offset};
use thiserror::Error;

use crate::{parse, CIFParseError, Record};

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("I/O:")]
    Io(#[from] std::io::Error),
    #[error("Parsing CIF:")]
    CIFParseError(CIFParseError<'static>),
    #[error("UTF-8:")]
    UTF8(std::str::Utf8Error),
    #[error("Parsing number:")]
    InvalidNumber(lexical_core::Error),
    #[error("Error:")]
    Other(String),
}

pub type ReaderResult<T> = std::result::Result<T, ReaderError>;

struct ReadBuf<R> {
    inner: R,
    buf: BytesMut,
}

pub struct Reader<R> {
    buf: ReadBuf<R>,
}

impl<R: Read> ReadBuf<R> {
    fn new(inner: R) -> Self {
        let buf = BytesMut::new();
        ReadBuf { inner, buf }
    }

    fn refill_until_eof(&mut self) -> ReaderResult<bool> {
        let prev_start = self.buf.len();
        self.buf.resize(prev_start + 4096, 0);
        let nread = self.inner.read(&mut self.buf[prev_start..])?;
        trace!("Read {} bytes", nread);
        self.buf.truncate(prev_start + nread);
        Ok(nread > 0)
    }

    fn buf(&self) -> &[u8] {
        &*self.buf
    }

    fn consume(&mut self, size: usize) {
        let _ = self.buf.split_to(size);
    }
}

impl<R: Read> Reader<R> {
    pub fn new(rdr: R) -> Self {
        let buf = ReadBuf::new(rdr);
        Self { buf }
    }

    pub fn read_next<T>(
        &mut self,
        mut f: impl for<'a> FnMut(Record<'a>) -> T,
    ) -> ReaderResult<Option<T>> {
        loop {
            let buf = self.buf.buf();
            match parse(buf) {
                Ok((rest, Record::Unrecognised(val))) => {
                    warn!("Unrecognised: {:#?}", val);
                    let consumed = buf.offset(rest);
                    self.buf.consume(consumed);
                }

                Ok((rest, val)) => {
                    let consumed = buf.offset(rest);

                    let res = f(val);

                    self.buf.consume(consumed);
                    return Ok(Some(res));
                }

                Err(Err::Incomplete(need)) => {
                    debug!("Need more: {:?}", need);
                    if !self.buf.refill_until_eof()? {
                        break;
                    }
                }
                Err(Err::Error(err)) => return Err(ReaderError::from(err)),
                Err(Err::Failure(err)) => return Err(ReaderError::from(err)),
            };
        }

        Ok(None)
    }

    pub fn get_ref(&self) -> &R {
        &self.buf.inner
    }
}

impl From<CIFParseError<'_>> for ReaderError {
    fn from(src: CIFParseError<'_>) -> Self {
        match src {
            CIFParseError::Utf8(err) => ReaderError::UTF8(err),
            CIFParseError::InvalidNumber(err) => ReaderError::InvalidNumber(err),
            other => ReaderError::Other(other.to_string()),
        }
    }
}
