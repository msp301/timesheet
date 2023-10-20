use chrono::NaiveDate;

use timesheet::period;

#[test]
fn first_of_the_month() {
    test_period(1, "2023/01/01", "2023/01/01", "2023/02/01");
}

#[test]
fn months_with_more_days() {
    test_period(31, "2023/02/20", "2023/01/31", "2023/02/28");
}

#[test]
fn months_with_fewer_days() {
    test_period(31, "2023/10/20", "2023/09/30", "2023/10/31");
}

#[test]
fn leap_year() {
    test_period(29, "2024/02/20", "2024/01/29", "2024/02/29");
}

#[test]
fn non_leap_year() {
    test_period(29, "2023/02/20", "2023/01/29", "2023/02/28");
}

#[test]
fn year_boundary_ahead() {
    test_period(5, "2023/12/20", "2023/12/05", "2024/01/05");
}

#[test]
fn year_boundary_behind() {
    test_period(5, "2024/01/01", "2023/12/05", "2024/01/05");
}

fn test_period(period_start: u32, today: &str, expacted_start: &str, expected_end: &str) {
    let fmt = "%Y/%m/%d";

    let todays_date = NaiveDate::parse_from_str(today, fmt).unwrap();

    let got = period::period(period_start, todays_date);
    let expected = period::Period {
        start: NaiveDate::parse_from_str(expacted_start, fmt).unwrap(),
        end: NaiveDate::parse_from_str(expected_end, fmt).unwrap(),
    };

    assert_eq!(got, expected);
}
