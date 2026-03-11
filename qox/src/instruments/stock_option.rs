use chrono::{DateTime, Utc};
use crate::{core::period::{DayCountConvention, DefaultPeriodCalculator, PeriodCalculator}, evaluators::black_scholes::finite_difference::VanillaPayoff, solvers::{black_scholes::finite_difference::solver::{FdmConfig, Solver}, time_stepping::dimsim2::Dimsim2}, traits::{instrument::{Instrument, OptionInstrument, OptionType}, market_view::OptionMarketView, payoff::{Payoff, PayoffAsInitialCondition}, real::Real}};
use crate::traits::rate_curve::RateCurve;
use crate::traits::vol_surface::VolSurface;

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
            option_type
        }
    }
}

impl Instrument for StockOption {}

impl<T: Real> OptionInstrument<T, VanillaPayoff> for StockOption {

    fn strike(self) -> f64 {
        self.strike
    }

    fn option_type(self) -> crate::traits::instrument::OptionType {
        self.option_type
    }

    fn years_to_expiry(self) -> T {
        let now = Utc::now().date_naive();
        
        let expiry_date = self.expiry.date_naive();

        let calculator = DefaultPeriodCalculator;
        let years = calculator.year_fraction(
            now, 
            expiry_date, 
            DayCountConvention::Actual365Fixed,
        );

        Real::from_f64(years.0)
    }
    
    fn evaluate<M, RC, VS>(self, market_frame: &M) -> T 
        where
            T: Real,
            RC: RateCurve<T>,
            VS: VolSurface<T>,
            M: OptionMarketView<T, RC, VS> {

        let solver = Solver {
            config: FdmConfig {
                nodes: 1000,
                time_steps: 10,
            },
        };

        let rate = market_frame.rate_curve().zero_rate(self.years_to_expiry());
        let vol = market_frame.vol_surface().volatility(0.0, T::zero());

        let initial_condition = PayoffAsInitialCondition::new(<StockOption as OptionInstrument<T, VanillaPayoff>>::get_payoff(self));

        let stepper = Dimsim2::new();
        solver.solve(
            stepper,
            initial_condition,
            self.years_to_expiry(),
            market_frame.spot_price(),
            rate,
            vol,
        )
    }
    
    fn get_payoff(self) -> VanillaPayoff {
        VanillaPayoff {
            strike: self.strike,
            option_type: self.option_type,
        }
    }

}
