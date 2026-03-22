use crate::instruments::{Instrument, OptionInstrument, OptionType};
use crate::methods::finite_difference::meshers::uniform::UniformMesher1d;
use crate::methods::finite_difference::solver::{FdmConfig, Solver};
use crate::methods::step_policy::linear_policy::LinearPolicy;
use crate::methods::step_policy::unified_policy::UnifiedPolicy;
use crate::methods::time_stepping::butcher_jackiewicz2::ButcherJackiewicz2;
use crate::methods::time_stepping::input_vectors::InputVector;
use crate::methods::transforms::log::LogTransform;
use crate::processes::FdmProcess;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseStyle {
    American,
    European,
}

#[derive(Debug, Clone, Copy)]
pub struct StockOption {
    pub strike: f64,
    pub expiry: DateTime<Utc>,
    pub option_type: OptionType,
    pub exercise_style: ExerciseStyle,
}

impl StockOption {
    pub fn new(
        strike: f64,
        expiry: DateTime<Utc>,
        option_type: OptionType,
        exercise_style: ExerciseStyle,
    ) -> Self {
        Self {
            strike,
            expiry,
            option_type,
            exercise_style,
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
                time_steps: 11,
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

        let process =
            BlackScholesProcess::new(rate, vol, transform, DayCountConvention::Actual365Fixed);
        let stepper = ButcherJackiewicz2::new();

        // let obstacle_policy = PostProjectionPolicy {
        //     constraint: AmericanConstraint::new(initial_conditions),
        // };

        let operator = process.build_operator(&mesher);

        let policy = match self.exercise_style {
            ExerciseStyle::European => UnifiedPolicy::Linear(LinearPolicy::new(&operator)),
            ExerciseStyle::American => todo!(),
        };

        let vector = solver.solve(
            stepper,
            initial_conditions,
            &mesher,
            dt,
            solver.config,
            //market_frame.spot_price(),
            &policy,
            //obstacle_policy,
        );

        solver.interpolate(&mesher, vector.step_slice(0), market_frame.spot_price())

        // let scaled_time_deriv = vector.step_slice(1);

        // let dv_dtau_scaled =
        //     solver.interpolate(&mesher, scaled_time_deriv, market_frame.spot_price());
        // -(dv_dtau_scaled)
    }

    fn get_payoff(self) -> VanillaPayoff {
        VanillaPayoff {
            strike: self.strike,
            option_type: self.option_type,
        }
    }
}
