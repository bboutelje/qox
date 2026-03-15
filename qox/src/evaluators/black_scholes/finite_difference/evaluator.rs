use crate::market::market_frame::OptionMarketFrame;
use crate::solvers::black_scholes::finite_difference::meshing::uniform::UniformMesher1d;
use crate::solvers::black_scholes::finite_difference::process::BlackScholesProcess;
use crate::solvers::black_scholes::finite_difference::solver_old::FdmConfig;
use crate::solvers::black_scholes::finite_difference::solver_old::Solver;
use crate::solvers::black_scholes::finite_difference::transforms::log::LogTransform;
use crate::solvers::time_stepping::dimsim2::Dimsim2;
use crate::traits::payoff::{Payoff, PayoffAsInitialConditions};
use crate::traits::{
    instrument::OptionInstrument, pricing_engine::OptionEvaluable, rate_curve::RateCurve,
    real::Real, vol_surface::VolSurface,
};

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

        let stepper = Dimsim2::new();
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
