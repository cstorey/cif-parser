use std::{
    fs::File,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::Result;
use log::*;
use structopt::StructOpt;

use cif_parser::Reader;

#[derive(Debug, StructOpt)]
#[structopt(name = "cif-parser", about = "CIF file parser")]
struct Opts {
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(&f)?;
        info!("Parsing file: {:?}", f);

        let size = fp.metadata()?.len();
        let mut rdr = Reader::new(fp);

        while let Some(()) = rdr.read_next(|r| {
            debug!("Ok: {:#?}", r);
        })? {
            let off = rdr.get_ref().seek(SeekFrom::Current(0))?;
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
