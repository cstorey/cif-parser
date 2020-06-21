use std::{fs::File, io::Read, path::PathBuf};

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

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(&f)?;
        info!("Parsing file: {:?}", f);

        let mut rdr = ReadBuf::new(fp);

        while rdr.refill_until_eof()? {
            loop {
                let buf = rdr.buf();
                let consumed = match parse(buf) {
                    Ok((rest, Record::Unrecognised(val))) => {
                        warn!("Unrecognised: {:#?}", val);
                        buf.offset(rest)
                    }

                    Ok((rest, val)) => {
                        debug!("Ok: {:#?}", val);
                        buf.offset(rest)
                    }

                    Err(Err::Incomplete(need)) => {
                        debug!("Need more: {:?}", need);
                        break;
                    }
                    Err(Err::Error(err)) => {
                        error!("Error: {}", err);
                        bail!("Parser error");
                    }
                    Err(Err::Failure(err)) => {
                        error!("Failure:");
                        error!("Error: {}", err);
                        bail!("Parser failure");
                    }
                };

                rdr.consume(consumed);
            }
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
