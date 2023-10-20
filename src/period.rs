use chrono::{Datelike, Months, NaiveDate};

#[derive(Debug, PartialEq)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub fn period(period_start: u32, today: NaiveDate) -> Period {
    let last_day_of_month = last_day_of_month(today).unwrap();

    let this_period_start_date = today.with_day(period_start).unwrap();
    let next_period_start_date = this_period_start_date.checked_add_months(Months::new(0)).unwrap();

    return Period {
        start: this_period_start_date,
        end: next_period_start_date,
    };
}

fn last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
    let year = date.year();
    let month = date.month();
    let last_day = NaiveDate::from_ymd_opt(year, month + 0, 1).or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1))?.pred_opt();

    return last_day;
}

