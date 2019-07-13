use env_logger;
use log::*;
use std::fs::File;
use std::path::PathBuf;

use failure::*;
use memmap::Mmap;
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
                    info!("Ok: {:#?}", val)
                }
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    bail!(e);
                }
                Err(nom::Err::Incomplete(e)) => {
                    bail!("Incomplete read: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
