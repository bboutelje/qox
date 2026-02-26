use crate::market::vol_surface::VolSurface;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::Real;

#[derive(Debug, Clone)]
pub struct OptionMarketData<T, RC, VS>
where
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    pub spot_price: T,
    pub rate_curve: RC,
    pub vol_surface: VS,
}

#[derive(Debug, Clone)]
pub struct MarketData<T, RC>
where
    T: Real,
    RC: RateCurve<T>,
{
    pub spot_price: T,
    pub rate_curve: RC,
}

impl<T, RC> MarketData<T, RC>
where
    T: Real,
    RC: RateCurve<T>,
{
    pub fn new(spot_price: T, rate_curve: RC) -> Self {
        Self {
            spot_price,
            rate_curve,
        }
    }
}


impl<T, RC, VS> OptionMarketData<T, RC, VS>
where
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    pub fn new(spot_price: T, rate_curve: RC, vol_surface: VS) -> Self {
        Self {
            spot_price: spot_price,
            rate_curve,
            vol_surface,
        }
    }
}