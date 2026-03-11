use crate::solvers::black_scholes::finite_difference::solver::{FdmConfig, Solver};
use crate::solvers::time_stepping::crank_nicolson::{CrankNicolson};
use crate::solvers::time_stepping::dimsim2::{Dimsim2};
//use crate::solvers::time_stepping::dimsim2::{self, Dimsim2};
use crate::solvers::time_stepping::sdirk22::{Sdirk22};
use crate::traits::payoff::{Payoff, PayoffAsInitialCondition};
use crate::{market::market_frame::OptionMarketFrame};
use crate::traits::{instrument::OptionInstrument, pricing_engine::OptionEvaluable, rate_curve::RateCurve, real::Real, vol_surface::VolSurface};

#[derive(Debug, Clone, Copy)]
pub struct Evaluator {
    pub config: FdmConfig,
}

impl<T, RC, VS, I, P> OptionEvaluable<T, RC, VS, I, P> for Evaluator
where
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
    I: OptionInstrument<T, P> + Copy,
    P: Payoff<T> + Copy,
{
    fn evaluate(self, instrument: I, market: OptionMarketFrame<T, RC, VS>) -> T {

        let solver = Solver {
            config: FdmConfig {
                nodes: self.config.nodes,
                time_steps: self.config.time_steps,
            },
        };

        let rate = market.rate_curve.zero_rate(instrument.years_to_expiry());
        let vol = market.vol_surface.volatility(0.0, T::zero());

        let initial_condition = PayoffAsInitialCondition::new(instrument.get_payoff());

        let stepper = Dimsim2::new();
        solver.solve(
            stepper,
            initial_condition,
            instrument.years_to_expiry(),
            market.spot_price,
            rate,
            vol,
        )
    }

}