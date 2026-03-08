use std::time::Instant;

use chrono::{Duration, Utc};
use qox::evaluators::black_scholes::finite_difference::evaluator::Evaluator;

use qox::real::dual::Dual;
use qox::real::dual_array::DualArray;
use qox::real::num_dual_vec::NumDualVec;
use qox::solvers::black_scholes::finite_difference::solver::FdmConfig;
use qox::traits::real::Real;
use qox::{instruments::future_option::{FutureOption, OptionType}, market::{market_data::OptionMarketData, rate_curve::ContinuousRateCurve, vol_surface::FlatVolSurface}};
use qox::traits::pricing_engine::OptionEvaluable;

pub fn main() {

    // let vol = NumDualVec::<2>::var(0.2, 0);
    // let rate = NumDualVec::<2>::var(0.05, 1);

    let vol = DualArray::<2>::var(0.2, 0); 
    let rate = DualArray::<2>::var(0.05, 1);

    let vol = Dual::var(0.2);

    let spot = 95.0;
    let vol = 0.2;
    let rate = 0.05;
    
    let market = OptionMarketData::new(
        spot,
        ContinuousRateCurve::new(rate),
        FlatVolSurface::new(vol),
    );

    let evaluator = Evaluator {
        config: FdmConfig {
            nodes: 1000,
            damping_steps: 0,
            time_steps: 10,
        }
    };

    let option = FutureOption::new(
        100.0,
        Utc::now() + Duration::days(365),
        OptionType::Call,
    );

    let start = Instant::now();

    let mut result_price = evaluator.evaluate(option, market);
    let n = 1;

    // for _ in 0..n {
    //     //ReverseGradient::reset_tape();
    //     result_price = evaluator.evaluate(option, market);
    // }
    let duration = start.elapsed();

    println!("Price: {:.4}", result_price.scalar());
    // let vega = result_price.0.eps.unwrap_generic(Const::<2>, nalgebra::U1)[0];
    // println!("Vega:  {:.4}", vega);

    //println!("Vega:  {:.4}", result_price.grad);
    //println!("Price: {:.4}", result_price.0.re());
    println!("Time taken: {:?}", duration / n as u32);
    // Note: Even though we focused on price, result_price is a Dual2Vec64.
    // result_price.eps[0] IS your Delta. It's already there!
}