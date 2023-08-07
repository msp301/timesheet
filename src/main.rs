use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Error, Write};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Datelike, Local};
use clap::{Parser, Subcommand};
use ordinal::Ordinal;
use regex::Regex;
use rev_lines::RevLines;

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
    End,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let command = &args.command;
    let filepath = args.path.expect("timesheet file required");

    return match command {
        Command::Parse => render_timesheet(filepath),
        Command::Start { task } => start_task(filepath, &task),
        Command::End => start_task(filepath, &"END".to_string()),
    }
}

fn start_task(filepath: std::path::PathBuf, task: &String) -> Result<(), Error> {
    let read_fh = File::open(&filepath).unwrap();
    let rev_lines = RevLines::new(&read_fh);

    let mut found_date_heading = false;
    let mut latest_date = Local::now().date_naive();
    for line in rev_lines {
        let current_line = line.expect("Failed to read line");
        let date = parse_date_heading(&current_line);
        if date.is_some() {
            latest_date = date.unwrap();
            found_date_heading = true;
            break;
        }
    }

    let write_fh = OpenOptions::new().append(true).open(&filepath).expect("Failed to open timesheet file");

    let todays_date = Local::now().date_naive();
    if !found_date_heading || (found_date_heading && !latest_date.eq(&todays_date)) {
        writeln!(&write_fh, "\n## {}\n", format_weekday(todays_date))?;
    }

    let time_now = Local::now().naive_local();
    let task_line = format!("{} - {}", time_now.format("%H:%M"), task);
    writeln!(&write_fh, "{}", task_line)?;
    println!("{}", task_line);

    Ok(())
}

fn render_timesheet(filepath: std::path::PathBuf) -> Result<(), Error> {
    let entries = parse_timesheet(filepath).unwrap();

    let mut previous_date: Option<NaiveDate> = None;

    let mut index = 0;
    let mut iter = entries.iter();
    while let Some(entry) = iter.next() {
        let start = entry.start;
        let current_date = NaiveDate::from(start);

        if previous_date.is_none() || !previous_date.unwrap().eq(&current_date) {
            println!("\n{}\n", format_weekday(current_date));
        }

        let stub_entry = Entry { start: Local::now().naive_local(), name: "Stub".to_string() };
        let next = entries.get(index + 1).unwrap_or(&stub_entry);

        let duration = next.start.signed_duration_since(start);
        let duration_str = format_jira_tempo(duration.num_minutes());
        let task = &entry.name;

        println!("{:<6} {}", duration_str, task);

        let next_task = &next.name;
        if next_task == "END" {
            iter.next();
            index += 1;
        }

        previous_date = Some(current_date);

        index += 1;
    }

    Ok(())
}

struct Entry {
    start: NaiveDateTime,
    name: String,
}

fn parse_timesheet(filepath: std::path::PathBuf) -> Result<Vec<Entry>, Error> {
    let fh = File::open(&filepath)?;
    let reader = BufReader::new(fh);

    let mut parsed_entries: Vec<Entry> = vec![];

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

        let time = parse_task_time(&current_line);
        if time.is_none() {
            continue;
        }

        let date_time = NaiveDateTime::new(current_date.unwrap(), time.unwrap());
        let task = extract_task(&current_line);

        parsed_entries.push(Entry { start: date_time, name: task });
    }

    Ok(parsed_entries)
}

fn parse_date_heading(str : &String) -> Option<NaiveDate> {
    let h2_regex = Regex::new(r"## \w+ (\d{1,2})\w{2} (\w+) (\d{4})").unwrap();
    if h2_regex.is_match(&str) {
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

    let captures = task_regex.captures(&str).unwrap();
    let time_str = captures.get(1).unwrap().as_str();

    return Some(NaiveTime::parse_from_str(time_str, "%H:%M").unwrap());
}

fn extract_task(str: &String) -> String {
    let result = str.split_once("-").unwrap().1;
    return result.trim().to_string();
}

fn format_weekday(date: NaiveDate) -> String {
    let formatted_date = format!("{} {} {}", date.weekday(), Ordinal(date.day()), date.format("%B %Y"));
    return formatted_date;
}

fn format_jira_tempo(mins: i64) -> String {
    let task_duration = mins as f64 / 60.0;

    let hours = task_duration as i64;
    let mins = (task_duration.fract() * 60.0) as i64;

    let mut time_str = "".to_owned();
    if hours > 0 {
        let hours_str = format!("{}h", hours);
        time_str.push_str(&hours_str);
    }
    if mins > 0 {
        let mins_str = format!("{}m", mins);
        time_str.push_str(&mins_str);
    }

    return time_str;
}
