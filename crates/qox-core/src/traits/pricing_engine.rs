use crate::market::market_data::{MarketData, OptionMarketData};
use crate::market::vol_surface::VolSurface;
use crate::traits::instrument::Instrument;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::Real;

pub trait PricingEngine<I, T, RC>
where
    I: Instrument,
    T: Real,
    RC: RateCurve<T>,
{
    fn price(
        &self,
        instrument: &I,
        market: &MarketData<T, RC>,
    ) -> T;
}

pub struct OptionPricingResult<T: Real> {
    pub price: T,
    pub delta: T,
    pub gamma: T,
    pub vega: T,
    pub theta: T,
    pub rho: T,
}


pub trait OptionPricingEngine<I, T, RC, VS>
where
    I: Instrument,
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    fn price(
        &self,
        instrument: &I,
        market: &OptionMarketData<T, RC, VS>,
    ) -> T;
    // fn price_and_greeks(
    //     &self,
    //     instrument: &I,
    //     market: &OptionMarketData<T, RC, VS>,
    // ) -> OptionPricingResult<T>;

    fn price_and_greeks(
        &self,
        instrument: &I,
        market: &OptionMarketData<T, RC, VS>,
    ) -> OptionPricingResult<f64>
    where
        RC: RateCurve<T>,
        VS: VolSurface<T>;
}