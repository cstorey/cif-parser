use std::io::Read;

use bytes::BytesMut;
use log::*;
use thiserror::Error;

use crate::{
    Association, BasicSchedule, CIFParseError, ChangeEnRoute, Header, LocationIntermediate,
    LocationOrigin, LocationTerminating, Record, ScheduleExtra, TiplocAmend, TiplocInsert, Trailer,
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

    pub fn read_next<T>(&mut self, mut f: impl FnMut(Record) -> T) -> ReaderResult<Option<T>> {
        const SNIPPET: usize = 128;
        loop {
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
                    return Ok(None);
                } else {
                    continue;
                }
            }

            let record = self.buf.split_to(CIF_LINE_LEN).freeze();
            let val = match &record[0..2] {
                b"HD" => Record::Header(Header::from_record(record)),
                b"TI" => Record::TiplocInsert(TiplocInsert::from_record(record)),
                b"TA" => Record::TiplocAmend(TiplocAmend::from_record(record)),
                b"AA" => Record::Association(Association::from_record(record)),
                b"BS" => Record::Schedule(BasicSchedule::from_record(record)),
                b"BX" => Record::ScheduleExtra(ScheduleExtra::from_record(record)),
                b"LO" => Record::LocationOrigin(LocationOrigin::from_record(record)),
                b"LI" => Record::LocationIntermediate(LocationIntermediate::from_record(record)),
                b"LT" => Record::LocationTerminating(LocationTerminating::from_record(record)),
                b"CR" => Record::ChangeEnRoute(ChangeEnRoute::from_record(record)),
                b"ZZ" => Record::Trailer(Trailer::from_record(record)),
                _ => Record::Unrecognised(record),
            };

            let res = f(val);
            return Ok(Some(res));
        }
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
