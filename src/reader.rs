use std::io::Read;

use anyhow::{bail, Result};
use bytes::BytesMut;
use log::*;
use nom::{Err, Offset};

use crate::{parse, Record};

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

    fn refill_until_eof(&mut self) -> Result<bool> {
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
    ) -> Result<Option<T>> {
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
                Err(Err::Error(err)) => {
                    bail!("Parser error: {}", err);
                }
                Err(Err::Failure(err)) => {
                    bail!("Parser failure: {}", err);
                }
            };
        }

        Ok(None)
    }

    pub fn get_ref(&self) -> &R {
        &self.buf.inner
    }
}
