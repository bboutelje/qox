use crate::{
    core::period::{DayCountConvention, DefaultPeriodCalculator, PeriodCalculator},
    traits::{
        instrument::{Instrument, OptionInstrument, OptionType},
        market_view::OptionMarketView,
        payoff::Payoff,
        rate_curve::RateCurve,
        vol_surface::VolSurface,
    },
    types::Real,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub struct FutureOption<P> {
    pub strike: f64,
    pub expiry: DateTime<Utc>,
    pub option_type: OptionType,
    pub payoff: P,
}

impl<P> FutureOption<P> {
    pub fn new(strike: f64, expiry: DateTime<Utc>, option_type: OptionType, payoff: P) -> Self {
        Self {
            strike: strike,
            expiry,
            option_type,
            payoff,
        }
    }
}

impl<P> Instrument for FutureOption<P> {}

impl<T: Real, P: Payoff<T> + Copy> OptionInstrument<T, P> for FutureOption<P> {
    fn strike(self) -> f64 {
        self.strike
    }

    fn option_type(self) -> crate::traits::instrument::OptionType {
        self.option_type
    }

    fn years_to_expiry(self) -> T {
        let now = Utc::now().date_naive();
        let expiry_date = self.expiry.date_naive();

        let calculator = DefaultPeriodCalculator;
        let years = calculator.year_fraction(now, expiry_date, DayCountConvention::Actual365Fixed);

        Real::from_f64(years.0)
    }

    fn evaluate<M, RC, VS>(self, _market_frame: &M) -> T
    where
        RC: RateCurve<T>,
        VS: VolSurface<T>,
        M: OptionMarketView<T, RC, VS>,
    {
        todo!()
    }

    fn get_payoff(self) -> P {
        todo!()
    }
}
