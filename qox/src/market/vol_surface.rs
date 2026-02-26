use chrono::NaiveDate;

use crate::{core::{error::CurveError, period::{DayCountConvention, PeriodCalculator}, tenor::Tenor}, math::interpolate::LinearInterpolator, traits::real::Real};


/// Trait for volatility surfaces (generic over T for future AD support)
pub trait VolSurface<T> {
    /// Returns implied volatility for time to expiry t
    fn volatility(&self, t: &T) -> T;
}

#[derive(Debug, Clone)]
pub struct FlatVolSurface<T> {
    vol: T,
}

impl<T> FlatVolSurface<T> {
    pub fn new<I: Into<T>>(vol: I) -> Self {
        Self {
            vol: vol.into()
        }
    }
}

impl<T> VolSurface<T> for FlatVolSurface<T>
where T: Copy {
    fn volatility(&self, _t: &T) -> T {
        self.vol
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
