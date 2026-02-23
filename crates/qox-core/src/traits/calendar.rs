use chrono::{Datelike, NaiveDate, Weekday};
use crate::conventions::BusinessDayConvention;
use crate::tenor::Tenor;

pub trait Calendar: std::fmt::Debug {
    fn name(&self) -> &str;
    fn is_holiday(&self, _date: NaiveDate) -> bool { false }
    
    fn is_business_day(&self, date: NaiveDate) -> bool {
        let wd = date.weekday();
        let is_weekend = wd == Weekday::Sat || wd == Weekday::Sun;
        !is_weekend && !self.is_holiday(date)
    }

    fn advance_business_days(&self, mut date: NaiveDate, n: i32) -> NaiveDate 
    {
        let step = if n >= 0 { 1 } else { -1 };
        let mut count = n.abs();
        while count > 0 {
            date = if step > 0 {
                date.succ_opt().expect("Date overflow")
            } else {
                date.pred_opt().expect("Date underflow")
            };
            if self.is_business_day(date) {
                count -= 1;
            }
        }
        date
    }

    fn advance_period(
        &self,
        date: NaiveDate,
        period: Tenor,
        convention: BusinessDayConvention,
        _is_eom: bool,
    ) -> NaiveDate {
        let raw_date = period.advance(date); // Use the Period's own advance logic
        convention.adjust(raw_date, self)
    }
}



