use std::{convert::TryInto, io::Read};

use bytes::{Buf, BytesMut};
use fallible_iterator::FallibleIterator;
use thiserror::Error;
use tracing::{trace, Level};

use crate::{
    Association, BasicSchedule, ChangeEnRoute, Header, LocationIntermediate, LocationOrigin,
    LocationTerminating, Record, ScheduleExtra, TiplocAmend, TiplocInsert, Trailer,
};

pub(crate) const CIF_RECORD_LEN: usize = 80;
// 80 characters plus a newline
const CIF_LINE_LEN: usize = 81;

pub(crate) type CifLine = [u8; CIF_RECORD_LEN];

#[derive(Error, Debug)]
pub enum ReaderError {
    #[error("I/O:")]
    Io(#[from] std::io::Error),
    #[error("Invalid record at byte: {}", 0)]
    InvalidRecord(usize),
}

pub type ReaderResult<T> = std::result::Result<T, ReaderError>;

struct Filler<R> {
    inner: R,
}

pub struct Reader<R> {
    src: Filler<R>,
    buf: BytesMut,
    offset: usize,
}

impl<R: Read> Filler<R> {
    const BUF_FILL_SIZE: usize = 64 * 1024;

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
        let offset = 0;
        Self { src, buf, offset }
    }

    pub fn read_next(&mut self) -> ReaderResult<Option<Record>> {
        const SNIPPET: usize = 128;
        loop {
            if tracing::enabled!(Level::TRACE) {
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
                trace!("Need more");
                if !self.src.refill_until_eof(&mut self.buf)? {
                    return Ok(None);
                } else {
                    continue;
                }
            }

            if self.buf[80] != b'\n' {
                return Err(ReaderError::InvalidRecord(self.offset));
            }

            let tmp = &self.buf[0..CIF_RECORD_LEN];
            assert!(
                tmp.len() == CIF_RECORD_LEN,
                "tmp:{} == CIF_RECORD_LEN:{}",
                tmp.len(),
                CIF_LINE_LEN,
            );
            let record: CifLine = tmp.try_into().expect("should be infallible");
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
            self.offset += CIF_LINE_LEN;
            self.buf.advance(CIF_LINE_LEN);

            return Ok(Some(val));
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.src.inner
    }
}

impl<R: Read> FallibleIterator for Reader<R> {
    type Item = Record;

    type Error = ReaderError;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.read_next()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_fail_if_newline_too_soon() {
        let it: &[u8] =
            b"ZZ                                                                             \n ";
        assert_ne!(it[80], b'\n');
        assert_eq!(it[79], b'\n');

        let mut r = Reader::new(it);

        let res = r.read_next();
        let e = res.unwrap_err();
        assert!(
            matches!(e, ReaderError::InvalidRecord(0)),
            "{:?} should be invalid record at 0",
            e
        );
    }
    #[test]
    fn should_fail_if_newline_too_late() {
        let it: &[u8] =
            b"ZZ                                                                               \n";
        assert_ne!(it[80], b'\n');
        assert_eq!(it[81], b'\n');

        let mut r = Reader::new(it);

        let res = r.read_next();
        let e = res.unwrap_err();
        assert!(
            matches!(e, ReaderError::InvalidRecord(0)),
            "{:?} should be invalid record at 0",
            e
        );
    }
    #[test]
    fn invalid_record_indicates_file_offset() {
        let it: &[u8] = b"\
        ZZ                                                                              \n\
        ZZ                                                                             \n ";
        assert_eq!(it[80], b'\n');
        assert_eq!(it[160], b'\n'); // One off
        assert_ne!(it[161], b'\n');

        let mut r = Reader::new(it);

        let _ = r.read_next().unwrap();
        let res = r.read_next();
        let e = res.unwrap_err();
        assert!(
            matches!(e, ReaderError::InvalidRecord(81)),
            "{:?} should be invalid record at 81",
            e
        );
    }
    #[test]
    fn invalid_record_indicates_file_offset_2() {
        let it: &[u8] = b"\
        ZZ                                                                              \n\
        ZZ                                                                              \n\
        ZZ                                                                              \n\
        ZZ                                                                             \n ";
        assert_eq!(it[80], b'\n');
        assert_eq!(it[161], b'\n');
        assert_eq!(it[242], b'\n');
        assert_eq!(it[322], b'\n'); // One off
        assert_ne!(it[323], b'\n');

        let mut r = Reader::new(it);

        let _ = r.read_next().unwrap();
        let _ = r.read_next().unwrap();
        let _ = r.read_next().unwrap();
        let res = r.read_next();
        let e = res.unwrap_err();
        assert!(
            matches!(e, ReaderError::InvalidRecord(243)),
            "{:?} should be invalid record at 243",
            e
        );
    }
}
