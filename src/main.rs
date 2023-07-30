use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let fh = File::open(&args.path)?;
    let reader = BufReader::new(fh);

    for line in reader.lines() {
        println!("{}", line?);
    }

    println!("Hello, world!");

    Ok(())
}
