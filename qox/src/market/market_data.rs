use crate::market::vol_surface::VolSurface;
use crate::real::dual_vec::DualVec64;
use crate::real::dual2_vec::Dual2Vec64;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::Real;

use std::marker::PhantomData;

pub struct PureMode;
pub struct DualVecMode;
pub struct Dual2VecMode;

// 2. Permission Trait
pub trait InMode<M> {}

// 3. Permissions
impl InMode<PureMode> for f64 {}
impl InMode<DualVecMode> for f64 {}
impl InMode<Dual2VecMode> for f64 {}

// Your specific type
impl<const N: usize> InMode<DualVecMode> for DualVec64<N> {}
impl<const N: usize> InMode<Dual2VecMode> for Dual2Vec64<N> {}


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