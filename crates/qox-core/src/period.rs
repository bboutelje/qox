// src/engine.rs
use chrono::{NaiveDate, Datelike};
pub use crate::conventions::{
    Thirty360Subtype,
    DayCountConvention,
};
use crate::{Days, Years};

pub struct DefaultPeriodCalculator;

/// Only handles integer day counts
pub trait PeriodCalculator<'a> {
    fn days_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        convention: DayCountConvention<'a>,
    ) -> Days;

    fn year_fraction(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        convention: DayCountConvention<'a>,
    ) -> Years;
}

impl<'a> PeriodCalculator<'a> for DefaultPeriodCalculator {
    fn days_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        convention: DayCountConvention<'a>,
    ) -> Days {
        calculate_days(start, end, convention)
    }

    fn year_fraction(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        convention: DayCountConvention<'a>,
    ) -> Years {
        if start == end {
            return Years(0.0);
        }

        if end < start {
            return Years(-self.year_fraction(end, start, convention).0);
        }

        match convention {
            DayCountConvention::Actual360 => {
                Years((end - start).num_days() as f64 / 360.0)
            }

            DayCountConvention::Actual365Fixed => {
                Years((end - start).num_days() as f64 / 365.0)
            }

            DayCountConvention::Thirty360(subtype) => {
                let days = calculate_days(start, end, DayCountConvention::Thirty360(subtype)).0 as f64;
                Years(days / 360.0)
            }

            DayCountConvention::Business252(calendar) => {
                let days =
                    calculate_days(start, end, DayCountConvention::Business252(calendar)).0 as f64;
                Years(days / 252.0)
            }

            DayCountConvention::ActActISDA => {
                let mut current = start;
                let mut acc = 0.0;

                while current < end {
                    let year_end =
                        NaiveDate::from_ymd_opt(current.year(), 12, 31).unwrap();
                    let next_year_start = year_end.succ_opt().unwrap();

                    let segment_end = if end < next_year_start {
                        end
                    } else {
                        next_year_start
                    };

                    let days = (segment_end - current).num_days() as f64;
                    let denom = if is_leap_year(current.year()) {
                        366.0
                    } else {
                        365.0
                    };

                    acc += days / denom;
                    current = segment_end;
                }

                Years(acc)
            }
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Private helper to keep the math DRY
fn calculate_days<'a>(
    start: NaiveDate,
    end: NaiveDate,
    convention: DayCountConvention<'a>,
) -> Days {
    let d = match convention {
        DayCountConvention::Actual360
        | DayCountConvention::Actual365Fixed
        | DayCountConvention::ActActISDA => {
            (end - start).num_days()
        }

        DayCountConvention::Business252(calendar) => {
            let mut count = 0;
            let mut current = start;

            while current < end {
                if calendar.is_business_day(current) {
                    count += 1;
                }
                current = current.succ_opt().expect("Date overflow");
            }

            count
        }

        DayCountConvention::Thirty360(subtype) => {
            let y1 = start.year() as i64;
            let m1 = start.month() as i64;
            let mut d1 = start.day() as i64;

            let y2 = end.year() as i64;
            let m2 = end.month() as i64;
            let mut d2 = end.day() as i64;

            match subtype {
                Thirty360Subtype::US => {
                    if d1 == 31 {
                        d1 = 30;
                    }

                    if d2 == 31 && d1 == 30 {
                        d2 = 30;
                    }
                }

                Thirty360Subtype::European => {
                    if d1 == 31 {
                        d1 = 30;
                    }
                    if d2 == 31 {
                        d2 = 30;
                    }
                }

                Thirty360Subtype::German => {
                    let is_last_feb = |date: NaiveDate| -> bool {
                        date.month() == 2
                            && date.day()
                                == if NaiveDate::from_ymd_opt(date.year(), 2, 29).is_some() {
                                    29
                                } else {
                                    28
                                }
                    };

                    if is_last_feb(start) {
                        d1 = 30;
                    }
                    if d1 == 31 {
                        d1 = 30;
                    }
                    if is_last_feb(end) {
                        d2 = 30;
                    }
                    if d2 == 31 {
                        d2 = 30;
                    }
                }
            }

            360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)
        }
    };

    Days(d)
}

