use chrono::{Datelike, Months, NaiveDate};

#[derive(Debug, PartialEq)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

pub fn period(period_start: u32, today: NaiveDate) -> Period {
    let last_day_of_month = get_last_day_of_month(today).unwrap();

    let mut this_period_start_date = today;

    if period_start < today.day() {
        this_period_start_date = this_period_start_date.with_day(period_start).unwrap();
    } else {
        if period_start > last_day_of_month.day() {
            let previous_month = today.checked_sub_months(Months::new(1)).unwrap();
            let last_day_of_previous_month = get_last_day_of_month(previous_month).unwrap();

            this_period_start_date = this_period_start_date
                .with_day(last_day_of_previous_month.day())
                .expect("No last day of month")
                .checked_sub_months(Months::new(1))
                .unwrap();
        }
    }

    let next_period_start_date = this_period_start_date
        .checked_add_months(Months::new(1))
        .unwrap();

    return Period {
        start: this_period_start_date,
        end: next_period_start_date,
    };
}

fn get_last_day_of_month(date: NaiveDate) -> Option<NaiveDate> {
    let year = date.year();
    let month = date.month();
    let last_day = NaiveDate::from_ymd_opt(year, month + 0, 1)
        .or_else(|| NaiveDate::from_ymd_opt(year + 1, 1, 1))?
        .pred_opt();

    return last_day;
}
