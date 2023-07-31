use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use chrono::NaiveDate;
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let fh = File::open(&args.path)?;
    let reader = BufReader::new(fh);

    for line in reader.lines() {
        let current_line = line?;
        if current_line.starts_with("## ") {
            println!("Found h2: {}", current_line);
            let h2_regex = Regex::new(r"## (\w{3}) (\d{1,2})\w{2} (\w+) (\d{4})").unwrap();
            if h2_regex.is_match(&current_line) {
                println!("Matched line: {}", current_line);

                let captures = h2_regex.captures(&current_line).unwrap();
                let day = captures.get(2).unwrap().as_str();
                let month = captures.get(3).unwrap().as_str();
                let year = captures.get(4).unwrap().as_str();

                let date_str = format!("{} {} {}", day, month, year);
                let dt = NaiveDate::parse_from_str(&date_str, "%d %B %Y").unwrap();

                println!("{}", dt);
            }
        }
    }

    Ok(())
}
