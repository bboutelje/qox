use chrono::NaiveDate;

use crate::math::interpolate::Interpolator1D;
use crate::types::Real;
use crate::{
    core::{
        error::CurveError,
        period::{DayCountConvention, PeriodCalculator},
        tenor::Tenor,
    },
    math::interpolate::LinearInterpolator,
    traits::vol_surface::VolSurface,
};

#[derive(Debug, Clone, Copy)]
pub struct FlatVolSurface<T> {
    vol: T,
}

impl<T> FlatVolSurface<T> {
    pub fn new(vol: T) -> Self {
        Self { vol: vol }
    }
}

impl<T: Real> VolSurface<T> for FlatVolSurface<T> {
    fn volatility(&self, _strike: f64, _t: T) -> T {
        self.vol.clone()
    }
}

#[derive(Clone)]
pub struct InterpolatedVolSurface<T: Real> {
    #[allow(dead_code)]
    reference_date: NaiveDate,
    #[allow(dead_code)]
    tenors: Vec<Tenor>,
    _vols: Vec<T>,
    _interpolator: LinearInterpolator<T>,
}

impl<T: Real> InterpolatedVolSurface<T> {
    pub fn new(
        reference_date: NaiveDate,
        tenors: Vec<Tenor>,
        vols: Vec<T>,
        calculator: &dyn PeriodCalculator,
    ) -> Result<Self, CurveError> {
        if tenors.len() != vols.len() {
            return Err(CurveError::LengthMismatch);
        }

        let year_fractions: Vec<T> = tenors
            .iter()
            .map(|tenor| {
                let end_date = tenor.advance(reference_date);
                let yf = calculator
                    .year_fraction(reference_date, end_date, DayCountConvention::Actual365Fixed)
                    .0;

                // Lift the f64 result into the generic type T
                T::from_f64(yf)
            })
            .collect();

        let interpolator = LinearInterpolator::new(year_fractions, vols.clone())?;

        Ok(Self {
            reference_date,
            tenors,
            _vols: vols,
            _interpolator: interpolator,
        })
    }
}

impl<T: Real> VolSurface<T> for InterpolatedVolSurface<T> {
    fn volatility(&self, _strike: f64, t: T) -> T {
        // Use your internal interpolator to compute the volatility
        self._interpolator.interpolate(t)
    }
}
