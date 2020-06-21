use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{bail, Result};
use bytes::{BufMut, BytesMut};
use log::*;
use nom::{Err, Offset};
use structopt::StructOpt;

use cif_parser::{parse, Record};

#[derive(Debug, StructOpt)]
#[structopt(name = "cif-parser", about = "CIF file parser")]
struct Opts {
    files: Vec<PathBuf>,
}
fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let mut fp = File::open(&f)?;
        info!("Parsing file: {:?}", f);

        let mut buf = BytesMut::new();
        loop {
            let prev_start = buf.len();
            buf.put(&[0u8; 4096] as &[u8]);
            let nread = fp.read(&mut buf[prev_start..])?;
            buf.truncate(prev_start + nread);
            if nread == 0 {
                break;
            }
            trace!("Read {}bytes of {}/{}", nread, &buf.len(), buf.capacity());

            loop {
                trace!("Buf len: {}; capacity: {}", buf.len(), buf.capacity());

                let view_len = 96;
                let (view, ellip) = if buf.len() < view_len {
                    (String::from_utf8_lossy(&buf), "")
                } else {
                    (String::from_utf8_lossy(&buf[0..view_len]), "…")
                };
                trace!("Parsing: {}{}", view, ellip);

                let consumed = match parse(&buf) {
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

                let _ = buf.split_to(consumed);
            }
        }
    }

    info!("Done.");

    Ok(())
}
