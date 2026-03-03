use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::solvers::black_scholes::finite_difference::solver::{FdmConfig, Solver};
use crate::traits::Resolved;
use crate::traits::payoff::{InitialCondition};
use crate::{market::market_data::OptionMarketData};
use crate::traits::{EvaluationResolver, instrument::OptionInstrument, pricing_engine::OptionEvaluable, rate_curve::RateCurve, real::Real, vol_surface::VolSurface};

pub struct Evaluator {
    pub config: FdmConfig,
}

impl<I, SReal, TResult, RC, VS> OptionEvaluable<I, SReal, VS::T, TResult, RC, VS> for Evaluator
where
    I: OptionInstrument,
    RC: RateCurve,
    VS: VolSurface,
    SReal: Real + PartialOrd + EvaluationResolver<RC::T, VS::T, Output = TResult>,
    TResult: Real + PartialOrd + 
        From<SReal> + From<I::T> + From<RC::T> +
        From<VS::T> + From<TResult> +
        Neg<Output = TResult>,
    // <SReal as EvaluationResolver<RC::T, VS::T>>::Output: Real,
    for<'a> &'a I: InitialCondition<Resolved<SReal, RC::T, VS::T>>,

    for<'a> &'a SReal: Add<&'a SReal, Output = SReal> + 
                       Sub<&'a SReal, Output = SReal> + 
                       Mul<&'a SReal, Output = SReal> + 
                       Div<&'a SReal, Output = SReal>,

    // Math bounds for the Rate type (RC::T)
    for<'a> &'a RC::T: Add<&'a RC::T, Output = RC::T> + 
                       Sub<&'a RC::T, Output = RC::T> + 
                       Mul<&'a RC::T, Output = RC::T> + 
                       Div<&'a RC::T, Output = RC::T>,

    // Math bounds for the Vol type
    for<'a> &'a VS::T: Add<&'a VS::T, Output = VS::T> + 
                        Sub<&'a VS::T, Output = VS::T> + 
                        Mul<&'a VS::T, Output = VS::T> + 
                        Div<&'a VS::T, Output = VS::T>,

    // Math bounds for the Output type (T)
    for<'a> &'a <SReal as EvaluationResolver<RC::T, VS::T>>::Output: 
    Add<Output = <SReal as EvaluationResolver<RC::T, VS::T>>::Output> + 
    Sub<Output = <SReal as EvaluationResolver<RC::T, VS::T>>::Output> + 
    Mul<Output = <SReal as EvaluationResolver<RC::T, VS::T>>::Output> + 
    Div<Output = <SReal as EvaluationResolver<RC::T, VS::T>>::Output> +
    Neg<Output = <SReal as EvaluationResolver<RC::T, VS::T>>::Output>,
{
    type Result = TResult;//<SReal as EvaluationResolver<RC::T, VS::T>>::Output;

    fn evaluate(&self, instrument: &I, market: &OptionMarketData<SReal, RC, VS>) -> TResult {
        // 1. Initialize the Black-Scholes Solver with current config
        // Note: Using the crate::path to where your Solver is defined
        let solver = Solver {
            config: FdmConfig {
                nodes: self.config.nodes,
                time_steps: self.config.time_steps,
                damping_steps: self.config.damping_steps,
            },
        };

        let rate = market.rate_curve.zero_rate(&RC::T::zero());
        let vol = market.vol_surface.volatility(&VS::T::zero());


        solver.solve(
            &instrument, 
            instrument.time_to_expiry(), 
            &market.spot_price,
            rate,
            vol,

        )
    }

}