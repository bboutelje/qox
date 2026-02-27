use crate::traits::rate_curve::RateCurve;
use crate::traits::real::{Real};
use crate::traits::vol_surface::VolSurface;

use std::marker::PhantomData;


#[derive(Debug, Clone)]
pub struct OptionMarketData<SReal, RC, VS>
where
    SReal: Real,
    RC: RateCurve,
    VS: VolSurface,
{
    pub spot_price: SReal,
    pub rate_curve: RC,
    pub vol_surface: VS,
}

#[derive(Debug, Clone)]
pub struct MarketData<T, RC>
where
    T: Real,
    RC: RateCurve<T = T>,
{
    pub spot_price: T,
    pub rate_curve: RC,
}

impl<T, RC> MarketData<T, RC>
where
    T: Real,
    RC: RateCurve<T = T>,
{
    pub fn new(spot_price: T, rate_curve: RC) -> Self {
        Self {
            spot_price,
            rate_curve,
        }
    }
}


impl<SReal, RC, VS> OptionMarketData<SReal, RC, VS>
where
    SReal: Real,
    RC: RateCurve,
    VS: VolSurface,
{
    pub fn new(spot_price: SReal, rate_curve: RC, vol_surface: VS) -> Self {
        Self {
            spot_price: spot_price,
            rate_curve,
            vol_surface,
        }
    }
}