use crate::instruments::{Instrument, OptionInstrument, OptionType};
use crate::methods::complementarity::brennan_schwartz::BrennanSchwartz;
use crate::methods::constraints::american::AmericanConstraint;
use crate::methods::finite_difference::free_boundary::projection::ProjectionConstrained;

use crate::methods::finite_difference::free_boundary::unconstrained::Unconstrained;
use crate::methods::finite_difference::meshers::uniform::UniformMesher1d;
use crate::methods::finite_difference::solver::{FdmConfig, Solver};
use crate::methods::obstacle_policies::ObstaclePolicy;
use crate::methods::obstacle_policies::brennan_schwartz::BrennanSchwartzPolicy;
use crate::methods::obstacle_policies::no_obstacle::NoObstaclePolicy;
use crate::methods::obstacle_policies::post_projection::PostProjectionPolicy;
use crate::methods::time_stepping::butcher_jackiewicz2::ButcherJackiewicz2;
use crate::methods::transforms::log::LogTransform;
use crate::processes::black_scholes::BlackScholesProcess;
use crate::traits::rate_curve::RateCurve;
use crate::traits::vol_surface::VolSurface;
use crate::types::Real;
use crate::{
    core::period::{DayCountConvention, DefaultPeriodCalculator, PeriodCalculator},
    evaluators::black_scholes::finite_difference::VanillaPayoff,
    traits::{market_view::OptionMarketView, payoff::PayoffAsInitialConditions},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub struct StockOption {
    pub strike: f64,
    pub expiry: DateTime<Utc>,
    pub option_type: OptionType,
}

impl StockOption {
    pub fn new(strike: f64, expiry: DateTime<Utc>, option_type: OptionType) -> Self {
        Self {
            strike: strike,
            expiry,
            option_type,
        }
    }
}

impl Instrument for StockOption {}

impl<T: Real> OptionInstrument<T, VanillaPayoff> for StockOption {
    fn strike(self) -> f64 {
        self.strike
    }

    fn option_type(self) -> OptionType {
        self.option_type
    }

    fn years_to_expiry(self) -> T {
        let now = Utc::now().date_naive();

        let expiry_date = self.expiry.date_naive();

        let calculator = DefaultPeriodCalculator;
        let years = calculator.year_fraction(now, expiry_date, DayCountConvention::Actual365Fixed);

        Real::from_f64(years.0)
    }

    fn evaluate<M, RC, VS>(self, market_frame: &M) -> T
    where
        T: Real,
        RC: RateCurve<T>,
        VS: VolSurface<T>,
        M: OptionMarketView<T, RC, VS>,
    {
        let solver = Solver {
            config: FdmConfig {
                nodes: 1000,
                time_steps: 12,
            },
        };

        let rate = market_frame.rate_curve().zero_rate(self.years_to_expiry());
        let vol = market_frame.vol_surface().volatility(0.0, T::zero());

        let dt = <StockOption as OptionInstrument<T, VanillaPayoff>>::years_to_expiry(self)
            / T::from_f64(solver.config.time_steps as f64);
        let initial_conditions = PayoffAsInitialConditions::new(
            <StockOption as OptionInstrument<T, VanillaPayoff>>::get_payoff(self),
        );
        let transform = LogTransform::new();
        let s_min = T::from_f64(0.01);
        let s_max = market_frame.spot_price() * T::from_f64(5.0);
        let mesher = UniformMesher1d::new(s_min.ln(), s_max.ln(), solver.config.nodes, transform);

        let process = BlackScholesProcess::new(rate, vol, transform);
        let stepper = ButcherJackiewicz2::new();

        let obstacle_policy = PostProjectionPolicy {
            constraint: AmericanConstraint::new(initial_conditions),
        };

        solver.solve(
            process,
            stepper,
            initial_conditions,
            mesher,
            dt,
            solver.config,
            market_frame.spot_price(),
            //NoObstaclePolicy,
            obstacle_policy,
        )
    }

    fn get_payoff(self) -> VanillaPayoff {
        VanillaPayoff {
            strike: self.strike,
            option_type: self.option_type,
        }
    }
}
