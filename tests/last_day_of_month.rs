use chrono::NaiveDate;
use timesheet::period::get_last_day_of_month;

#[test]
fn january() {
    assert_eq!(
        last_day_of_month(2024, 01, 04),
        NaiveDate::from_ymd_opt(2024, 01, 31).unwrap()
    );
}

#[test]
fn february() {
    assert_eq!(
        last_day_of_month(2024, 02, 04),
        NaiveDate::from_ymd_opt(2024, 02, 29).unwrap()
    );

    assert_eq!(
        last_day_of_month(2023, 02, 04),
        NaiveDate::from_ymd_opt(2023, 02, 28).unwrap()
    );
}

#[test]
fn march() {
    assert_eq!(
        last_day_of_month(2024, 03, 04),
        NaiveDate::from_ymd_opt(2024, 03, 31).unwrap()
    );
}

#[test]
fn april() {
    assert_eq!(
        last_day_of_month(2024, 04, 04),
        NaiveDate::from_ymd_opt(2024, 04, 30).unwrap()
    );
}

#[test]
fn may() {
    assert_eq!(
        last_day_of_month(2024, 05, 04),
        NaiveDate::from_ymd_opt(2024, 05, 31).unwrap()
    );
}

#[test]
fn june() {
    assert_eq!(
        last_day_of_month(2024, 06, 04),
        NaiveDate::from_ymd_opt(2024, 06, 30).unwrap()
    );
}

#[test]
fn july() {
    assert_eq!(
        last_day_of_month(2024, 07, 04),
        NaiveDate::from_ymd_opt(2024, 07, 31).unwrap()
    );
}

#[test]
fn august() {
    assert_eq!(
        last_day_of_month(2024, 08, 04),
        NaiveDate::from_ymd_opt(2024, 08, 31).unwrap()
    );
}

#[test]
fn september() {
    assert_eq!(
        last_day_of_month(2024, 09, 04),
        NaiveDate::from_ymd_opt(2024, 09, 30).unwrap()
    );
}

#[test]
fn october() {
    assert_eq!(
        last_day_of_month(2024, 10, 04),
        NaiveDate::from_ymd_opt(2024, 10, 31).unwrap()
    );
}

#[test]
fn november() {
    assert_eq!(
        last_day_of_month(2024, 11, 04),
        NaiveDate::from_ymd_opt(2024, 11, 30).unwrap()
    );
}

#[test]
fn december() {
    assert_eq!(
        last_day_of_month(2024, 12, 04),
        NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    );
}

fn last_day_of_month(year: i32, month: u32, day: u32) -> NaiveDate {
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    get_last_day_of_month(date).unwrap()
}
