use crate::market::market_data::{MarketData, OptionMarketData};
use crate::market::vol_surface::VolSurface;
use crate::traits::instrument::Instrument;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::Real;

pub trait Evaluable<I, T, RC>
where
    I: Instrument,
    T: Real,
    RC: RateCurve<T>,
{
    fn evaluate(
        &self,
        instrument: &I,
        market: &MarketData<T, RC>,
    ) -> T;
}

pub struct OptionEvaluation<T: Real> {
    pub price: T,
    pub delta: T,
    pub gamma: T,
    pub vega: T,
    pub theta: T,
    pub rho: T,
}


pub trait OptionEvaluable<I, T, RC, VS>
where
    I: Instrument,
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    fn evaluate(
        &self,
        instrument: &I,
        market: &OptionMarketData<T, RC, VS>,
    ) -> T;
    // fn price_and_greeks(
    //     &self,
    //     instrument: &I,
    //     market: &OptionMarketData<T, RC, VS>,
    // ) -> OptionEvaluation<T>;

    fn evaluate_all(
        &self,
        instrument: &I,
        market: &OptionMarketData<T, RC, VS>,
    ) -> OptionEvaluation<f64>
    where
        RC: RateCurve<T>,
        VS: VolSurface<T>;
}