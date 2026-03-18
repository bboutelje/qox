pub mod cash;
pub mod future_option;
pub mod stock_option;

use crate::{
    traits::{
        market_view::OptionMarketView, payoff::Payoff, rate_curve::RateCurve,
        vol_surface::VolSurface,
    },
    types::Real,
};

pub trait Instrument {}

#[derive(Debug, Clone, Copy)]
pub enum OptionType {
    Call,
    Put,
}

pub trait OptionInstrument<T: Real, P: Payoff<T> + Copy>: Instrument {
    fn strike(self) -> f64;
    fn option_type(self) -> OptionType;
    fn years_to_expiry(self) -> T;

    fn get_payoff(self) -> P;

    fn evaluate<M, RC, VS>(self, market_frame: &M) -> T
    where
        M: OptionMarketView<T, RC, VS>,
        RC: RateCurve<T>,
        VS: VolSurface<T>;
}

pub trait FutureInstrument<T: Real>: Instrument {
    fn delivery_time(&self) -> T;
    fn forward_price(&self) -> T;
}
