use chrono::{Duration, Utc};
use qox::{instruments::stock_option::{self, StockOption}, market::{market_frame::{self, MarketFrame, OptionMarketFrame}, rate_curve::ContinuousRateCurve, vol_surface::FlatVolSurface}, traits::{instrument::{OptionInstrument, OptionType}, market_view}};


pub fn main(){

    let stock_option = StockOption::new(
        100.0,
        Utc::now() + Duration::days(365),
        OptionType::Call,
    );

    let spot = 95.0;
    let rate = 0.05;
    let vol = 0.2;

    let market_frame = OptionMarketFrame::new(
        spot,
        ContinuousRateCurve::new(rate),
        FlatVolSurface::new(vol),
    );

    let price = stock_option.evaluate(&market_frame);

    println!("Price: {}", price)
}