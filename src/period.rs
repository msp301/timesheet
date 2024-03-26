use core::f64;
use std::i64;

use chrono::{Datelike, Months, NaiveDate, Weekday};

#[derive(Debug, PartialEq)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub fn period(period_start: u32, today: NaiveDate) -> Period {
    let previous_month = today.checked_sub_months(Months::new(1)).unwrap();
    let last_day_previous_month = get_last_day_of_month(previous_month).unwrap();
    let last_day_month = get_last_day_of_month(today).unwrap();

    let this_period_start_date = match period_start {
        i if i == today.day() => today,
        i if i < today.day() => today.with_day(period_start).unwrap(),
        i if i < last_day_previous_month.day() => previous_month.with_day(period_start).unwrap(),
        i if i > last_day_previous_month.day() => today.with_day(1).unwrap(),
        _ => last_day_previous_month,
    };

    let last_day_of_next_month = get_last_day_of_month(
        this_period_start_date
            .checked_add_months(Months::new(1))
            .unwrap(),
    )
    .unwrap();

    let end_day = match period_start {
        i if i == 1 => last_day_month.day(),
        i if i > last_day_month.day() => last_day_of_next_month.day(),
        _ => period_start - 1,
    };

    let start_month = this_period_start_date.month();
    let this_month = today.month();

    let period_end_date = match period_start {
        i if i == 1 => today.with_day(end_day).unwrap(),
        i if i > this_period_start_date.day() => today.with_day(end_day).unwrap(),
        i if i <= last_day_of_next_month.day() && start_month != this_month => {
            today.with_day(end_day).unwrap()
        }
        _ => last_day_of_next_month
            .with_day(end_day)
            .unwrap_or_else(|| last_day_of_next_month),
    };

    return Period {
        start: this_period_start_date,
        end: period_end_date,
    };
}

pub fn get_work_time_in_period(period: &Period) -> i64 {
    let workdays = get_work_days_in_period(period);
    let workday_hours = 7.5;

    (workdays as f64 * workday_hours * 60.0).round() as i64
}

pub fn get_work_days_in_period(period: &Period) -> i64 {
    let period_duration = period.end.signed_duration_since(period.start);
    let days = period_duration.num_days() + 1;

    let mut workdays = 0;
    for i in 0..days {
        let date = period
            .start
            .checked_add_days(chrono::Days::new(i as u64))
            .unwrap();

        if is_work_day(date.weekday()) {
            workdays += 1;
        }
    }

    return workdays;
}

pub fn is_work_day(day: Weekday) -> bool {
    match day {
        Weekday::Mon => true,
        Weekday::Tue => true,
        Weekday::Wed => true,
        Weekday::Thu => true,
        Weekday::Fri => true,
        _ => false,
    }
}

pub fn get_last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
    let year = date.year();
    let month = date.month();
    let last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1))?
        .pred_opt();

    return last_day;
}
