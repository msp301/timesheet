use chrono::NaiveDate;

use timesheet::period;

#[test]
fn first_of_the_month() {
    test_period(1, "2023/01/01", "2023/01/01", "2023/01/31");
}

#[test]
fn months_with_more_days() {
    test_period(31, "2023/02/20", "2023/01/31", "2023/02/28");
}

#[test]
fn months_with_fewer_days() {
    test_period(31, "2023/10/20", "2023/10/01", "2023/10/30");
}

#[test]
fn leap_year() {
    test_period(29, "2024/02/20", "2024/01/29", "2024/02/28");
    test_period(29, "2024/02/28", "2024/01/29", "2024/02/28");
    test_period(29, "2024/02/29", "2024/02/29", "2024/03/28");
}

#[test]
fn non_leap_year() {
    test_period(29, "2023/02/20", "2023/01/29", "2023/02/28");
    test_period(29, "2023/02/28", "2023/01/29", "2023/02/28");
    test_period(29, "2023/03/01", "2023/03/01", "2023/03/28");
}

#[test]
fn year_boundary_ahead() {
    test_period(5, "2023/12/20", "2023/12/05", "2024/01/04");
}

#[test]
fn year_boundary_behind() {
    test_period(5, "2024/01/01", "2023/12/05", "2024/01/04");
}

#[test]
fn periods_do_not_overlap_month_start() {
    test_period(1, "2024/01/01", "2024/01/01", "2024/01/31");
    test_period(1, "2024/02/01", "2024/02/01", "2024/02/29");
    test_period(1, "2024/03/01", "2024/03/01", "2024/03/31");
    test_period(1, "2024/04/01", "2024/04/01", "2024/04/30");
}

#[test]
fn periods_do_not_overlap_month_middle() {
    test_period(18, "2024/01/15", "2023/12/18", "2024/01/17");
    test_period(18, "2024/01/18", "2024/01/18", "2024/02/17");
    test_period(18, "2024/02/18", "2024/02/18", "2024/03/17");
    test_period(18, "2024/03/18", "2024/03/18", "2024/04/17");
}

#[test]
fn periods_do_not_overlap_month_end() {
    test_period(31, "2024/01/15", "2023/12/31", "2024/01/30");
    test_period(31, "2024/01/31", "2024/01/31", "2024/02/29");
    test_period(31, "2024/03/01", "2024/03/01", "2024/03/30");
    test_period(31, "2024/03/31", "2024/03/31", "2024/04/30");
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
