use chrono::{DateTime, Utc};
use crate::{traits::{instrument::{Instrument, OptionInstrument}, real::Real}};

#[derive(Debug, Clone, Copy)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone)]
pub struct FutureOption<T: Real> {
    pub strike: T,
    pub expiry: DateTime<Utc>,
    pub option_type: OptionType,
}

impl<T: Real> FutureOption<T> {
    pub fn new<I: Into<T>>(strike: I, expiry: DateTime<Utc>, option_type: OptionType) -> Self {
        Self {
            strike: strike.into(),
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

impl<T: Real> Instrument<T> for FutureOption<T> {}

impl<T: Real> OptionInstrument<T> for FutureOption<T> {
    fn strike(&self) -> T {
        self.strike.clone()
    }

    fn is_call(&self) -> bool {
        matches!(self.option_type, OptionType::Call)
    }

    fn time_to_expiry(&self) -> T {
        let now = Utc::now();
        let duration = self.expiry.signed_duration_since(now);
        let seconds = duration.num_seconds() as f64;
        let years = seconds / (365.25 * 24.0 * 3600.0);
        T::from_f64(years)
    }
}
