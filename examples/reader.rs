use std::{fs::File, io::Seek, path::PathBuf};

use anyhow::Result;
use fallible_iterator::FallibleIterator;
use structopt::StructOpt;

use cif_parser::Reader;
use tracing::{debug, info};

#[derive(Debug, StructOpt)]
#[structopt(name = "cif-parser", about = "CIF file parser")]
struct Opts {
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::from_args();

    for f in opts.files {
        let fp = File::open(&f)?;
        info!("Parsing file: {:?}", f);

        let size = fp.metadata()?.len();
        let mut rdr = Reader::new(fp);

        while let Some(r) = rdr.next()? {
            println!("{:?}", r);
            let off = rdr.get_ref().stream_position()?;
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
