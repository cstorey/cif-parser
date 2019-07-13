use env_logger;
use log::*;
use std::fs::File;
use std::path::PathBuf;

use failure::*;
use memmap::Mmap;
use nom::{error::VerboseError, Err};
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
        let fp = File::open(f)?;
        let mmap = unsafe { Mmap::map(&fp)? };
        let mut i: &[u8] = &mmap;
        while i.len() > 0 {
            match parse::<VerboseError<_>>(&i) {
                Ok((rest, val)) => {
                    i = rest;
                    info!("Ok: {:#?}", val)
                }

                Err(Err::Incomplete(need)) => {
                    error!("Needed: {:?}", need);
                    return Result::Err(failure::err_msg("Not enough data"));
                }
                Err(Err::Error(err)) => {
                    error!("Error:");
                    show_error(err);
                    return Result::Err(failure::err_msg("Parser error"));
                }
                Err(Err::Failure(err)) => {
                    error!("Failure:");
                    show_error(err);
                    return Result::Err(failure::err_msg("Parser failure"));
                }
            }
        }
    }

    Ok(())
}

fn show_error(err: VerboseError<&[u8]>) {
    const SNIPPET_LEN: usize = 240;
    for (i, kind) in err.errors {
        let len = std::cmp::min(i.len(), SNIPPET_LEN);
        error!(
            "Err: {:?}: {:?}{}",
            kind,
            String::from_utf8_lossy(&i[..len]),
            if i.len() < SNIPPET_LEN { "" } else { "…" }
        );
    }
}
