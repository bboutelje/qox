// tests/thirty360_us.rs

use chrono::{Datelike, NaiveDate};
use qox_core::period::{DefaultPeriodCalculator, PeriodCalculator};
use qox_core::conventions::{DayCountConvention, Thirty360Subtype};
use qox_core::traits::calendar::Calendar;

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}


#[test]
fn thirty360_us_start_is_31() {
    // Arrange
    let calc = DefaultPeriodCalculator;
    let start = date(2024, 1, 31);
    let end   = date(2024, 2, 28);

    let convention = DayCountConvention::Thirty360(Thirty360Subtype::US);

    // Act
    let days = calc.days_between(start, end, convention);

    // Assert
    // Formula: 360*(0) + 30*(2-1) + (28 - 30) = 28
    assert_eq!(days.0, 28);
}

#[test]
fn thirty360_us_end_is_31() {
    // Arrange
    let calc = DefaultPeriodCalculator;
    let start = date(2024, 1, 31);
    let end   = date(2024, 3, 31);

    let convention = DayCountConvention::Thirty360(Thirty360Subtype::US);
    

    // Act
    let days = calc.days_between(start, end, convention);

    // Assert
    // d1 adjusted to 30, d2 adjusted to 30 → 30*(3-1) + (30-30) = 60
    assert_eq!(days.0, 60);
}


#[test]
fn thirty360_us_reference_cases() {
    let calc = DefaultPeriodCalculator;

    let convention = DayCountConvention::Thirty360(Thirty360Subtype::US);
    

    // 1. Start = 2006-08-31, End = 2007-02-28
    let start = date(2006, 8, 31);
    let end   = date(2007, 2, 28);
    let days = calc.days_between(start, end, convention);
    assert_eq!(days.0, 178);  // QuantLib reference value

    // 2. Start = 2006-08-31, End = 2007-02-27
    let start = date(2006, 8, 31);
    let end   = date(2007, 2, 27);
    let days = calc.days_between(start, end, convention);
    assert_eq!(days.0, 177);  // QuantLib reference value

    // 3. Start = 2007-02-28, End = 2007-08-31
    let start = date(2007, 2, 28);
    let end   = date(2007, 8, 31);
    let days = calc.days_between(start, end, convention);
    assert_eq!(days.0, 183);  // QuantLib reference value

    // 4. Start = 2006-02-28, End = 2006-08-31
    let start = date(2006, 2, 28);
    let end   = date(2006, 8, 31);
    let days = calc.days_between(start, end, convention);
    assert_eq!(days.0, 183);  // QuantLib reference value
}

#[test]
fn act_act_isda_reference_case() {
    let calc = DefaultPeriodCalculator;

    let start = date(1999, 7, 1);
    let end   = date(2000, 7, 1);

    let convention = DayCountConvention::ActActISDA;

    let yf = calc.year_fraction(start, end, convention);

    // QuantLib reference: 184/365 + 182/366
    let expected = 184.0/365.0 + 182.0/366.0;

    assert!((yf.0 - expected).abs() < 1e-12);
}


#[test]
fn thirty360_german_basic_cases() {
    let calc = DefaultPeriodCalculator;
    let convention = DayCountConvention::Thirty360(Thirty360Subtype::German);
    

    // 1. Start = 2024-02-29 (leap year), End = 2024-03-31
    let start = date(2024, 2, 29);
    let end   = date(2024, 3, 31);
    let days = calc.days_between(start, end, convention);
    // d1 adjusted to 30 (last Feb), d2 adjusted to 30 → 30*(3-2) + (30-30) = 30
    assert_eq!(days.0, 30);

    // 2. Start = 2023-02-28 (non-leap), End = 2023-08-31
    let start = date(2023, 2, 28);
    let end   = date(2023, 8, 31);
    let days = calc.days_between(start, end, convention);
    // German 30/360 adjusts last Feb day to 30 → 30*(8-2) + (30-30) = 180
    assert_eq!(days.0, 180);

    // 3. Start = 2023-01-31, End = 2023-03-31
    let start = date(2023, 1, 31);
    let end   = date(2023, 3, 31);
    let days = calc.days_between(start, end, convention);
    // d1 = 30, d2 = 30 → 30*(3-1) + (30-30) = 60
    assert_eq!(days.0, 60);
}

#[test]
fn thirty360_european_basic_cases() {
    let calc = DefaultPeriodCalculator;
    let convention = DayCountConvention::Thirty360(Thirty360Subtype::European);
    

    // 1. Start = 2024-01-31, End = 2024-02-28
    let start = date(2024, 1, 31);
    let end   = date(2024, 2, 28);
    let days = calc.days_between(start, end, convention);
    // European 30/360: d1 = 30, d2 = 28 → 30*(2-1) + (28-30) = 28
    assert_eq!(days.0, 28);

    // 2. Start = 2024-01-30, End = 2024-03-31
    let start = date(2024, 1, 30);
    let end   = date(2024, 3, 31);
    let days = calc.days_between(start, end, convention);
    // d1 = 30, d2 = 30 → 30*(3-1) + (30-30) = 60
    assert_eq!(days.0, 60);

    // 3. Start = 2024-02-28, End = 2024-08-31
    let start = date(2024, 2, 28);
    let end   = date(2024, 8, 31);
    let days = calc.days_between(start, end, convention);
    // d1 = 28, d2 = 30 → 30*(8-2) + (30-28) = 182
    assert_eq!(days.0, 182);
}

#[derive(Debug)]
struct TestCalendar;
impl Calendar for TestCalendar {
    fn name(&self) -> &str {
        "TestCalendar"
    }
    fn is_business_day(&self, date: NaiveDate) -> bool {
        // Simple Monday-Friday calendar for testing
        let weekday = date.weekday();
        weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun
    }
}

#[test]
fn business252_basic() {
    let calc = DefaultPeriodCalculator;
    let cal = TestCalendar;
    let convention = DayCountConvention::Business252(&cal);

    let start = date(2024, 2, 1); // Thursday
    let end   = date(2024, 2, 10); // Saturday
    let days = calc.days_between(start, end, convention);
    // Feb 1,2,5,6,7,8,9 → 7 business days
    assert_eq!(days.0, 7);

    let yf = calc.year_fraction(start, end, convention);
    assert_eq!(yf.0, 7.0/252.0);
}
