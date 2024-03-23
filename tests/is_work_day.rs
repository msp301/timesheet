use timesheet::period::is_work_day;

#[test]
fn work_days_are_week_days() {
    assert!(is_work_day(chrono::Weekday::Mon));
    assert!(is_work_day(chrono::Weekday::Tue));
    assert!(is_work_day(chrono::Weekday::Wed));
    assert!(is_work_day(chrono::Weekday::Thu));
    assert!(is_work_day(chrono::Weekday::Fri));
    assert_eq!(is_work_day(chrono::Weekday::Sat), false);
    assert_eq!(is_work_day(chrono::Weekday::Sun), false);
}
