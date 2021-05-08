use std::io::Read;

use bytes::{Buf, BytesMut};
use log::*;
use nom::{Err, Offset};
use thiserror::Error;

use crate::{
    parse, Association, BasicSchedule, CIFParseError, Header, LocationOrigin, Record,
    ScheduleExtra, TiplocAmend, TiplocInsert, Trailer,
};

// 80 characters plus a newline
const CIF_LINE_LEN: usize = 81;

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

struct Filler<R> {
    inner: R,
}

pub struct Reader<R> {
    src: Filler<R>,
    buf: BytesMut,
}

impl<R: Read> Filler<R> {
    const BUF_FILL_SIZE: usize = 4096;

    fn new(inner: R) -> Self {
        Filler { inner }
    }

    fn refill_until_eof(&mut self, buf: &mut BytesMut) -> ReaderResult<bool> {
        let prev_start = buf.len();
        buf.resize(prev_start + Self::BUF_FILL_SIZE, 0);
        let nread = self.inner.read(&mut buf[prev_start..])?;
        trace!("Read {} bytes", nread);
        buf.truncate(prev_start + nread);
        Ok(nread > 0)
    }
}

impl<R: Read> Reader<R> {
    pub fn new(rdr: R) -> Self {
        let src = Filler::new(rdr);
        let buf = BytesMut::new();
        Self { src, buf }
    }

    pub fn read_next<T>(
        &mut self,
        mut f: impl for<'a> FnMut(Record<'a>) -> T,
    ) -> ReaderResult<Option<T>> {
        loop {
            const SNIPPET: usize = 128;
            if log::log_enabled!(log::Level::Trace) {
                trace!("Top of loop: buf.len(): {:?}", self.buf.len());

                if self.buf.len() > SNIPPET {
                    trace!(
                        "Buffer now: {:?}…",
                        String::from_utf8_lossy(&self.buf[..SNIPPET])
                    )
                } else {
                    trace!("Buffer now: {:?}", String::from_utf8_lossy(&self.buf))
                }
            }

            if self.buf.len() < CIF_LINE_LEN {
                debug!("Need more");
                if !self.src.refill_until_eof(&mut self.buf)? {
                    break;
                }
            }

            match &self.buf[0..2] {
                b"HD" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::Header(Header::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"TI" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::TiplocInsert(TiplocInsert::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"TA" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::TiplocAmend(TiplocAmend::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"AA" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::Association(Association::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"BS" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::Schedule(BasicSchedule::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"BX" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::ScheduleExtra(ScheduleExtra::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"LO" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::LocationOrigin(LocationOrigin::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                b"ZZ" => {
                    let record = self.buf.split_to(CIF_LINE_LEN).freeze();
                    let val = Record::Trailer(Trailer::from_record(record));
                    let res = f(val);
                    return Ok(Some(res));
                }
                _ => {}
            }

            let res = parse(&*self.buf);
            trace!("Result => {:?}", res);
            match res {
                Ok((rest, val)) => {
                    let consumed = self.buf.offset(rest);
                    debug!("Consumed: {:?}", consumed);

                    let res = f(val);

                    trace!("Pre advance: buf.len(): {:?}", self.buf.len());
                    self.buf.advance(consumed);
                    trace!("Post advance: buf.len(): {:?}", self.buf.len());
                    return Ok(Some(res));
                }

                Err(Err::Incomplete(need)) => {
                    debug!("Need more: {:?}", need);
                    if !self.src.refill_until_eof(&mut self.buf)? {
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
        &self.src.inner
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
