use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Error, Write};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use clap::{Parser, Subcommand};
use regex::Regex;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(global=true)]
    path: Option<std::path::PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Parse,
    Start {
        task: String,
    },
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let command = &args.command;
    let filepath = args.path.expect("timesheet file required");

    return match command {
        Command::Parse => parse_timesheet(filepath),
        Command::Start { task } => start_task(filepath, &task),
    }
}

fn start_task(filepath: std::path::PathBuf, task: &String) -> Result<(), Error> {
    let fh = OpenOptions::new().append(true).open(&filepath).expect("Failed to open timesheet file");

    // TODO: Confirm if date heading exists for today's date
    // TODO: Write date heading line (if needed)
    // TODO: Add task line with current time as start time
    write!(&fh, "TIME - {}", task)?;

    Ok(())
}

fn parse_timesheet(filepath: std::path::PathBuf) -> Result<(), Error> {
    let fh = File::open(&filepath)?;
    let reader = BufReader::new(fh);

    let mut current_date: Option<NaiveDate> = None;
    let mut previous_task_date_time: Option<NaiveDateTime> = None;

    for line in reader.lines() {
        let current_line = line?;
        if current_line.starts_with("## ") {
            current_date = parse_date_heading(&current_line);
            previous_task_date_time = None;
            continue;
        }

        if current_date.is_none() {
            continue;
        }

        let time = parse_task_time(&current_line);
        if time.is_none() {
            continue;
        }

        let date_time = NaiveDateTime::new(current_date.unwrap(), time.unwrap());

        if previous_task_date_time.is_none() {
            previous_task_date_time = Some(date_time);
            continue;
        }

        let duration = date_time.signed_duration_since(previous_task_date_time.unwrap());
        let task_duration = duration.num_minutes() as f64 / 60.0;

        let hours = task_duration as i64;
        let mins = (task_duration.fract() * 60.0) as i64;

        println!("{}h {}m", hours, mins);

        previous_task_date_time = Some(date_time);
    }

    Ok(())
}

fn parse_date_heading(str : &String) -> Option<NaiveDate> {
    let h2_regex = Regex::new(r"## \w+ (\d{1,2})\w{2} (\w+) (\d{4})").unwrap();
    if h2_regex.is_match(&str) {
        println!("Matched line: {}", str);

        let captures = h2_regex.captures(&str).unwrap();
        let day = captures.get(1).unwrap().as_str();
        let month = captures.get(2).unwrap().as_str();
        let year = captures.get(3).unwrap().as_str();

        let date_str = format!("{} {} {}", day, month, year);
        return Some(NaiveDate::parse_from_str(&date_str, "%d %B %Y").unwrap());
    }

    return None;
}

fn parse_task_time(str: &String) -> Option<NaiveTime> {
    let task_regex = Regex::new(r"(\d{2}:\d{2}) - (.*)").unwrap();
    if !task_regex.is_match(&str) {
        return None;
    }

    println!("Matched task: {}", &str);

    let captures = task_regex.captures(&str).unwrap();
    let time_str = captures.get(1).unwrap().as_str();

    return Some(NaiveTime::parse_from_str(time_str, "%H:%M").unwrap());
}
