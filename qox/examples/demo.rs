use chrono::{Duration, Utc};
use qox::instruments::{OptionInstrument, OptionType};
use qox::{
    instruments::stock_option::StockOption,
    market::{
        market_frame::OptionMarketFrame, rate_curve::ContinuousRateCurve,
        vol_surface::FlatVolSurface,
    },
};
use std::time::Instant;

pub fn main() {
    let stock_option = StockOption::new(100.0, Utc::now() + Duration::days(365), OptionType::Put);

    let spot = 95.0;
    let rate = 0.05;
    let vol = 0.2;

    let market_frame = OptionMarketFrame::new(
        spot,
        ContinuousRateCurve::new(rate),
        FlatVolSurface::new(vol),
    );
    let start = Instant::now();

    let price = stock_option.evaluate(&market_frame);

    let duration = start.elapsed();
    println!("Price: {:.8}", price);
    println!("Time taken: {:?}", duration);
}
