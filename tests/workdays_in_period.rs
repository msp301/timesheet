use chrono::NaiveDate;
use timesheet::period::{self, get_work_days_in_period};

#[test]
fn five_day_work_week() {
    test_work_days_in_period("2024/03/04", "2024/03/10", 5);
}

#[test]
fn work_days_within_month() {
    test_work_days_in_period("2024/01/01", "2024/01/31", 23);
}

#[test]
fn start_and_end_midweek() {
    test_work_days_in_period("2024/02/01", "2024/02/29", 21);
}

#[test]
fn spanning_months() {
    test_work_days_in_period("2024/01/15", "2024/02/15", 24);
}

#[test]
fn same_day() {
    test_work_days_in_period("2024/01/01", "2024/01/01", 1);
}

#[test]
fn bank_holidays() {
    test_work_days_in_period("2024/05/01", "2024/05/31", 21);
}

fn test_work_days_in_period(start: &str, end: &str, expected: i64) {
    let fmt = "%Y/%m/%d";

    let start_date = NaiveDate::parse_from_str(start, fmt).unwrap();
    let end_date = NaiveDate::parse_from_str(end, fmt).unwrap();

    let period = period::Period {
        start: start_date,
        end: end_date,
    };

    let got = get_work_days_in_period(&period);

    assert_eq!(got, expected);
}
