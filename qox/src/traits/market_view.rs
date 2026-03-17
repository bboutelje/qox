use crate::{
    traits::{rate_curve::RateCurve, vol_surface::VolSurface},
    types::Real,
};

pub trait MarketView<T: Real, RC: RateCurve<T>> {
    fn spot_price(&self) -> T;
    fn rate_curve(&self) -> &RC;
}

pub trait OptionMarketView<T: Real, RC: RateCurve<T>, VS: VolSurface<T>>:
    MarketView<T, RC>
{
    fn vol_surface(&self) -> &VS;
}
