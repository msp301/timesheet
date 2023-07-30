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
                // let day = captures.get(0).unwrap().as_str().parse::<u32>().unwrap();
                let day : u32 = captures.get(2).unwrap().as_str().parse::<u32>().unwrap().try_into().unwrap();
                let month_str = captures.get(3).unwrap().as_str();//.parse::<u32>().unwrap().try_into().unwrap();
                let year : u32 = captures.get(4).unwrap().as_str().parse::<u32>().unwrap().try_into().unwrap();

                println!("day: '{}', month: '{}', year: '{}'", day, month_str, year);

                //TODO: Parse 'month_str' into numeric month OR find a way to support parsing the
                //complete line into a DateTime that can support abbreviated dates.
                //Haven't been sure how to support parsing '1st' or '24th'.

                // let date_utc = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                // println!("Got date: {}", date_utc);

            }
        }
        // println!("{}", current_line);
    }

    Ok(())
}
