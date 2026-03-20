use crate::instruments::OptionInstrument;
use crate::market::market_frame::OptionMarketFrame;
use crate::methods::finite_difference::meshers::uniform::UniformMesher1d;
use crate::methods::finite_difference::solver_old::FdmConfig;
use crate::methods::finite_difference::solver_old_old::Solver;

use crate::methods::time_stepping::butcher_jackiewicz2::ButcherJackiewicz2;
use crate::methods::transforms::log::LogTransform;
use crate::processes_old::black_scholes::BlackScholesProcess;
use crate::traits::payoff::{Payoff, PayoffAsInitialConditions};
use crate::traits::{
    pricing_engine::OptionEvaluable, rate_curve::RateCurve, vol_surface::VolSurface,
};
use crate::types::Real;

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

        let initial_condition = PayoffAsInitialConditions::new(instrument.get_payoff());

        let stepper = ButcherJackiewicz2::new();
        let transform = LogTransform::new();
        let s_min = T::from_f64(0.01);
        let s_max = market.spot_price * T::from_f64(5.0);
        let _mesher = UniformMesher1d::new(s_min, s_max, solver.config.nodes, transform);
        let _process = BlackScholesProcess {
            r: rate,
            sigma: vol,
            transform,
        };
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
