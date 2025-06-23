use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;

mod header;

use header::{FileHeader, HeaderParsingError};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    root_file: PathBuf,
}

#[allow(dead_code)]
struct RootFile {
    header: FileHeader,
}

fn main() -> Result<(), HeaderParsingError> {
    let args = Args::parse();

    let mut reader = BufReader::new(File::open(&args.root_file).expect("Unable to open root_file"));

    let header = FileHeader::new(&mut reader)?;

    dbg!(header);

    Ok(())
}
