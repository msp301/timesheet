use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
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

    let mut current_date: Option<NaiveDate> = None;

    for line in reader.lines() {
        let current_line = line?;
        if current_line.starts_with("## ") {
            current_date = parse_date_heading(&current_line);
            continue;
        }

        if current_date.is_none() {
            continue;
        }

        let task_regex = Regex::new(r"(\d{2}:\d{2}) - (.*)").unwrap();
        if task_regex.is_match(&current_line) {
            println!("Matched task: {}", &current_line);

            let captures = task_regex.captures(&current_line).unwrap();
            let time_str = captures.get(1).unwrap().as_str();

            let date = current_date.unwrap();
            let time = NaiveTime::parse_from_str(time_str, "%H:%M").unwrap();

            let date_time = NaiveDateTime::new(date, time);
            println!("Parsed: {}", date_time)
        }
    }

    Ok(())
}

fn parse_date_heading(str : &String) -> Option<NaiveDate> {
    let h2_regex = Regex::new(r"## (\w{3}) (\d{1,2})\w{2} (\w+) (\d{4})").unwrap();
    if h2_regex.is_match(&str) {
        println!("Matched line: {}", str);

        let captures = h2_regex.captures(&str).unwrap();
        let day = captures.get(2).unwrap().as_str();
        let month = captures.get(3).unwrap().as_str();
        let year = captures.get(4).unwrap().as_str();

        let date_str = format!("{} {} {}", day, month, year);
        return Some(NaiveDate::parse_from_str(&date_str, "%d %B %Y").unwrap());
    }

    return None;
}
