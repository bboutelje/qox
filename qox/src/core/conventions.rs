use chrono::{NaiveDate, Datelike};

use crate::traits::calendar::Calendar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessDayConvention {
    Unadjusted,
    Following,
    Preceding,
    ModifiedFollowing,
    ModifiedPreceding,
}

#[derive(Debug, Clone, Copy)]
pub enum DayCountConvention<'a> {
    Actual360,
    Actual365Fixed,
    ActActISDA,
    Thirty360(Thirty360Subtype),
    Business252(&'a dyn Calendar),
}

impl<'a> DayCountConvention<'a> {}


// #[derive(Debug, Clone, Copy)]
// pub enum DayCountMethod<'a> {
//     /// Pure calendar math. No adjustments. 
//     /// Used for most options and simple interest.
//     Standard(DayCountBasis<'a>),

//     /// The "Business" rule. Adjusts start/end dates via a calendar
//     /// before applying the basis. Used for Swaps/Bonds.
//     Adjusted {
//         basis: DayCountBasis<'a>,
//         calendar: &'a dyn Calendar,
//         convention: BusinessDayConvention,
//     },
// }

// #[derive(Debug, Clone, Copy)]
// pub enum DayCountBasis<'a> {
//     Actual360,      // US Money Markets
//     Actual365Fixed, // US Treasuries / Options
//     Thirty360(Thirty360Subtype),
//     ActActISDA,
//     Business252(&'a dyn Calendar),
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Thirty360Subtype {
    US,
    European,
    German,
}

impl BusinessDayConvention {
    pub fn adjust<C: Calendar + ?Sized>(&self, date: NaiveDate, cal: &C) -> NaiveDate {
        match self {
            Self::Unadjusted => date,
            Self::Following => {
                let mut d = date;
                while !cal.is_business_day(d) {
                    d = d.succ_opt().unwrap();
                }
                d
            }
            Self::Preceding => {
                let mut d = date;
                while !cal.is_business_day(d) {
                    d = d.pred_opt().unwrap();
                }
                d
            }
            Self::ModifiedFollowing => {
                let next = Self::Following.adjust(date, cal);
                if next.month() != date.month() {
                    Self::Preceding.adjust(date, cal)
                } else {
                    next
                }
            },
            Self::ModifiedPreceding => {
                let prev = Self::Preceding.adjust(date, cal);
                if prev.month() != date.month() {
                    Self::Following.adjust(date, cal)
                } else {
                    prev
                }
            }
        }
    }
}

