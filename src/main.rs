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
fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let mut fp = File::open(&f)?;
        info!("Parsing file: {:?}", f);

        let mut buf = BytesMut::new();
        loop {
            let prev_start = buf.len();
            buf.resize(prev_start + 4096, 0);
            let nread = fp.read(&mut buf[prev_start..])?;
            buf.truncate(prev_start + nread);
            if nread == 0 {
                break;
            }

            loop {
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
