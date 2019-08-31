use env_logger;
use log::*;
use std::fs::File;
use std::path::PathBuf;

use failure::*;
use memmap::Mmap;
use nom::Err;
use structopt::StructOpt;

use cif_parser::parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "cif-parser", about = "CIF file parser")]
struct Opts {
    files: Vec<PathBuf>,
}
fn main() -> Fallible<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(&f)?;
        let mmap = unsafe { Mmap::map(&fp)? };
        let mut i: &[u8] = &mmap;
        info!("Parsing file: {:?}", f);
        while i.len() > 0 {
            match parse(&i) {
                Ok((rest, val)) => {
                    i = rest;
                    debug!("Ok: {:#?}", val)
                }

                Err(Err::Incomplete(need)) => {
                    error!("Needed: {:?}", need);
                    return Result::Err(failure::err_msg("Not enough data"));
                }
                Err(Err::Error(err)) => {
                    error!("Error: {}", err);
                    return Result::Err(failure::err_msg("Parser error"));
                }
                Err(Err::Failure(err)) => {
                    error!("Failure:");
                    error!("Error: {}", err);
                    return Result::Err(failure::err_msg("Parser failure"));
                }
            }
        }
    }

    info!("Done.");

    Ok(())
}
