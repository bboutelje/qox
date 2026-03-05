use std::ops::{Add, Div, Mul, Neg, Sub};
use std::time::Instant;

use crate::solvers::black_scholes::finite_difference::solver::{FdmConfig, Solver};
use crate::solvers::time_stepping::crank_nicholson::{self, DimsimCN};
use crate::solvers::time_stepping::implicit_euler::ImplicitEuler;
use crate::solvers::time_stepping::sdirk22::{self, Sdirk22};
use crate::{market::market_data::OptionMarketData};
use crate::traits::{EvaluationResolver, instrument::OptionInstrument, pricing_engine::OptionEvaluable, rate_curve::RateCurve, real::Real, vol_surface::VolSurface};

#[derive(Debug, Clone, Copy)]
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
{
    type Result = TResult;

    fn evaluate(self, instrument: I, market: OptionMarketData<SReal, RC, VS>) -> TResult {

        let solver = Solver {
            config: FdmConfig {
                nodes: self.config.nodes,
                time_steps: self.config.time_steps,
                damping_steps: self.config.damping_steps,
            },
        };

        let rate = market.rate_curve.zero_rate(&RC::T::zero());
        let vol = market.vol_surface.volatility(&VS::T::zero());

        let stepper = DimsimCN::new();
        solver.solve(
            stepper,
            instrument, 
            instrument.time_to_expiry(), 
            market.spot_price,
            rate,
            vol,
        )
    }

}