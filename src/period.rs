use chrono::{Datelike, Months, NaiveDate};

#[derive(Debug, PartialEq)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub fn period(period_start: u32, today: NaiveDate) -> Period {
    let mut this_period_start_date = today;

    if period_start < today.day() {
        this_period_start_date = this_period_start_date.with_day(period_start).unwrap();
    } else {
        let previous_month = today.checked_sub_months(Months::new(1)).unwrap();
        let last_day_of_previous_month = get_last_day_of_month(previous_month).unwrap();

        this_period_start_date = last_day_of_previous_month;
    }

    let mut next_start_day = period_start;
    let last_day_of_next_month = get_last_day_of_month(
        this_period_start_date
            .checked_add_months(Months::new(1))
            .unwrap(),
    )
    .unwrap();

    if next_start_day > last_day_of_next_month.day() {
        next_start_day = last_day_of_next_month.day()
    }

    let next_period_start_date = last_day_of_next_month.with_day(next_start_day).unwrap();

    return Period {
        start: this_period_start_date,
        end: next_period_start_date,
    };
}

pub fn get_last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
    let year = date.year();
    let month = date.month();
    let last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1))?
        .pred_opt();

    return last_day;
}
