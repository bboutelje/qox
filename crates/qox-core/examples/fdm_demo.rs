use std::time::Instant;

use chrono::{Duration, Utc};
use nalgebra::Const;
use num_dual::Dual2Vec;
use num_dual::DualStruct;
use qox::{engines::fdm::{FdmConfig, FdmEngine}, instruments::future_option::{FutureOption, OptionType}, market::{market_data::OptionMarketData, rate_curve::ContinuousRateCurve, vol_surface::FlatVolSurface}, real::dual2_vec::Dual2Vec64};
use qox::traits::pricing_engine::OptionPricingEngine;

pub fn main() {

    let spot = 95.0;
    let rate = 0.05;
    let vol = 0.2;


    // let spot = Dual2Vec64::<3>(Dual2Vec::from_re(95.0));
    // let rate = Dual2Vec64::<3>(Dual2Vec::from_re(0.05));
    // //let vol  = Dual2Vec64::<1>(Dual2Vec::from_re(0.20).derivative(0));
    
    // let vol = Dual2Vec64::<3>(<Dual2Vec<_, _, Const<3>>>::from_re(0.20).derivative(0));
    
    let market = OptionMarketData::new(
        spot,
        ContinuousRateCurve::new(rate),
        FlatVolSurface::new(vol),
    );

    // Initialize FDM Engine
    let engine = FdmEngine {
        config: FdmConfig {
            nodes: 200,
            damping_steps: 0,
            time_steps: 100,
        }
    };

    let option = FutureOption::new(
        100.0,
        Utc::now() + Duration::days(365),
        OptionType::Put,
    );

    let start = Instant::now();
    // This calls the FDM logic instead of the Black formula
    let result_price = engine.price(&option, &market);
    let duration = start.elapsed();

    println!("Price: {:.4}", result_price);

    // println!("Price: {:.4}", result_price.0.re());
    println!("Time taken: {:?}", duration);
    // Note: Even though we focused on price, result_price is a Dual2Vec64.
    // result_price.eps[0] IS your Delta. It's already there!
}