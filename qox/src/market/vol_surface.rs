use std::ops::{Add, Div, Mul, Sub};

use chrono::NaiveDate;

use crate::{core::{error::CurveError, period::{DayCountConvention, PeriodCalculator}, tenor::Tenor}, math::interpolate::LinearInterpolator, traits::{real::Real, vol_surface::VolSurface}};

#[derive(Debug, Clone)]
pub struct FlatVolSurface<T> {
    vol: T,
}

impl<T> FlatVolSurface<T> {
    pub fn new(vol: T) -> Self {
        Self {
            vol: vol
        }
    }
}

impl<T: Real> VolSurface for FlatVolSurface<T>
where for<'a> &'a T: Add<&'a T, Output = T> + 
                   Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + 
                   Div<&'a T, Output = T>, {
    type T = T;
    fn volatility(&self, _t: &T) -> T {
        self.vol.clone()
    }
}

pub struct InterpolatedVolSurface<T: Real> {
    #[allow(dead_code)]
    reference_date: NaiveDate,
    #[allow(dead_code)]
    tenors: Vec<Tenor>,
    _vols: Vec<T>,
    _interpolator: LinearInterpolator<T>,
}

impl<T: Real + PartialOrd> InterpolatedVolSurface<T> {
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
                    .year_fraction(
                        reference_date, 
                        end_date, 
                        DayCountConvention::Actual365Fixed
                    )
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

// impl VolSurface<f64> for InterpolatedVolSurface< {
//     fn volatility(&self, t: f64) -> f64 {
//         self.interpolator.interpolate(t)
//     }
// }
