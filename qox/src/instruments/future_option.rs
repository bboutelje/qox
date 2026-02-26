use chrono::{DateTime, Utc};
use crate::{core::period::{DayCountConvention, DefaultPeriodCalculator, PeriodCalculator}, traits::{instrument::{Instrument, OptionInstrument}, real::Real}};

#[derive(Debug, Clone, Copy)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone)]
pub struct FutureOption {
    pub strike: f64,
    pub expiry: DateTime<Utc>,
    pub option_type: OptionType,
}

impl FutureOption {
    pub fn new(strike: f64, expiry: DateTime<Utc>, option_type: OptionType) -> Self {
        Self {
            strike: strike,
            expiry,
            option_type,
        }
    }

    // pub fn payoff(&self, futures_price: T) -> T {
    //     match self.option_type {
    //         OptionType::Call => (futures_price - self.strike.clone()).max(T::from_f64(0.0)),
    //         OptionType::Put => (self.strike.clone() - futures_price).max(T::from_f64(0.0)),
    //     }
    // }

    // pub fn is_in_the_money(&self, futures_price: T) -> bool {
    //     match self.option_type {
    //         OptionType::Call => futures_price > self.strike.clone(),
    //         OptionType::Put => futures_price < self.strike.clone(),
    //     }
    // }
}

impl Instrument for FutureOption {}

impl<T: Real> OptionInstrument<T> for FutureOption {
    fn strike(&self) -> f64 {
        self.strike
    }

    fn is_call(&self) -> bool {
        matches!(self.option_type, OptionType::Call)
    }

    fn time_to_expiry(&self) -> T {
        // 1. Get current date in UTC and convert to NaiveDate
        let now = Utc::now().date_naive();
        
        // 2. Extract the NaiveDate from your expiry (assuming self.expiry is a DateTime or NaiveDate)
        let expiry_date = self.expiry.date_naive();

        // 3. Use the DefaultPeriodCalculator to get the year fraction
        // Note: 'convention' would likely be a field on your struct
        let calculator = DefaultPeriodCalculator;
        let years = calculator.year_fraction(
            now, 
            expiry_date, 
            DayCountConvention::Actual365Fixed,
        );

        // 4. Convert the wrapped f64 (Years) into your generic type T
        Real::from_f64(years.0)
    }
}
