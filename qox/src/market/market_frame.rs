use crate::traits::market_view::{MarketView, OptionMarketView};
use crate::traits::rate_curve::RateCurve;
use crate::traits::vol_surface::VolSurface;
use crate::types::Real;

#[derive(Debug, Clone)]
pub struct MarketFrame<T, RC>
where
    T: Real,
    RC: RateCurve<T>,
{
    pub spot_price: T,
    pub rate_curve: RC,
}

impl<T: Real, RC: RateCurve<T>> MarketView<T, RC> for MarketFrame<T, RC> {
    fn spot_price(&self) -> T {
        self.spot_price
    }

    fn rate_curve(&self) -> &RC {
        &self.rate_curve
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OptionMarketFrame<T, RC, VS>
where
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    pub spot_price: T,
    pub rate_curve: RC,
    pub vol_surface: VS,
}

impl<T, RC, VS> MarketView<T, RC> for OptionMarketFrame<T, RC, VS>
where
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    fn spot_price(&self) -> T {
        self.spot_price
    }

    fn rate_curve(&self) -> &RC {
        &self.rate_curve
    }
}

impl<T, RC, VS> OptionMarketView<T, RC, VS> for OptionMarketFrame<T, RC, VS>
where
    T: Real,
    RC: RateCurve<T>,
    VS: VolSurface<T>,
{
    fn vol_surface(&self) -> &VS {
        &self.vol_surface
    }
}

impl<T, RC, VS> OptionMarketFrame<T, RC, VS>
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
