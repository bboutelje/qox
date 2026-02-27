use crate::market::market_data::{MarketData, OptionMarketData};
use crate::traits::EvaluationResolver;
use crate::traits::instrument::Instrument;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::{Real};
use crate::traits::vol_surface::VolSurface;

pub trait Evaluable<I, T, RC>
where
    I: Instrument,
    T: Real,
    RC: RateCurve<T = T>,
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

pub trait OptionEvaluable<I, SReal, VsReal, T, RC, VS>
where
    I: Instrument,
    SReal: Real + EvaluationResolver<RC::T, VsReal, Output = T>,
    RC: RateCurve,
    VS: VolSurface,
    VsReal: Real,
    T: Real,
    // Use RC::T directly to link SReal to the RateCurve's internal type
    
{
    type Result: Real;
    fn evaluate(
        &self,
        instrument: &I,
        // Update the market data reference to use the associated type for the rate
        market: &OptionMarketData<SReal, RC, VS>,
    ) -> Self::Result;
}