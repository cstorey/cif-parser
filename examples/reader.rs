use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::{bail, Result};
use bytes::BytesMut;
use log::*;
use nom::{Err, Offset};
use structopt::StructOpt;

use cif_parser::{parse, Record};

#[derive(Debug, StructOpt)]
#[structopt(name = "cif-parser", about = "CIF file parser")]
struct Opts {
    files: Vec<PathBuf>,
}

struct ReadBuf<R> {
    inner: R,
    buf: BytesMut,
}

struct Reader<R> {
    buf: ReadBuf<R>,
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(&f)?;
        info!("Parsing file: {:?}", f);

        let size = fp.metadata()?.len();
        let mut rdr = Reader::new(ReadBuf::new(fp));

        while let Some(()) = rdr.read_next(|r| {
            debug!("Ok: {:#?}", r);
        })? {
            let off = rdr.buf.inner.seek(SeekFrom::Current(0))?;
            debug!(
                "{}/{}; {:.2}%",
                off,
                size,
                100.0 * (off as f64) / (size as f64)
            );
        }
    }

    info!("Done.");

    Ok(())
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
    fn new(buf: ReadBuf<R>) -> Self {
        Self { buf }
    }

    fn read_next<T>(&mut self, mut f: impl for<'a> FnMut(Record<'a>) -> T) -> Result<Option<T>> {
        while self.buf.refill_until_eof()? {
            loop {
                let buf = self.buf.buf();
                match parse(buf) {
                    Ok((rest, Record::Unrecognised(val))) => {
                        warn!("Unrecognised: {:#?}", val);
                        buf.offset(rest)
                    }

                    Ok((rest, val)) => {
                        debug!("Ok: {:#?}", val);
                        let consumed = buf.offset(rest);

                        let res = f(val);

                        self.buf.consume(consumed);
                        return Ok(Some(res));
                    }

                    Err(Err::Incomplete(need)) => {
                        debug!("Need more: {:?}", need);
                        break;
                    }
                    Err(Err::Error(err)) => {
                        bail!("Parser error: {}", err);
                    }
                    Err(Err::Failure(err)) => {
                        bail!("Parser failure: {}", err);
                    }
                };
            }
        }

        Ok(None)
    }
}
