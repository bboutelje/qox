use crate::market::market_frame::{MarketFrame, OptionMarketFrame};
use crate::traits::instrument::Instrument;
use crate::traits::payoff::Payoff;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::{Real};
use crate::traits::vol_surface::VolSurface;

pub trait Evaluable<I, T, RC>
where
    I: Instrument,
    T: Real,
    RC: RateCurve<T>,
{
    fn evaluate(
        &self,
        instrument: &I,
        market: &MarketFrame<T, RC>,
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

pub trait OptionEvaluable<T, RC, VS, I, P>
where
    T: Real,    
    RC: RateCurve<T>,
    VS: VolSurface<T>,
    I: Instrument,
    P: Payoff<T>,
{
    //type T: Real;
    fn evaluate(
        self,
        instrument: I,
        market: OptionMarketFrame<T, RC, VS>,
    ) -> T;
}