use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    root_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut reader = BufReader::new(File::open(&args.root_file).expect("Unable to open root_file"));

    println!("{:?}", reader);
}
