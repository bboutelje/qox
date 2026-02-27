use std::ops::{Add, Div, Mul, Neg, Sub};

use chrono::NaiveDate;
use crate::core::error::CurveError;
use crate::core::period::{DayCountConvention, PeriodCalculator};
use crate::core::tenor::Tenor;
use crate::math::interpolate::LinearInterpolator;
use crate::traits::rate_curve::RateCurve;
use crate::traits::real::Real;
use crate::core::rate::{Compounding, Frequency, InterestRate};
use crate::core::rate::Discountable;
use crate::math::interpolate::Interpolator1D;

#[derive(Debug, Clone)]
pub struct FlatRateCurve<'a, T: Real> {
    rate: InterestRate<'a, T>,
}

impl<'a, T: Real> FlatRateCurve<'a, T> {
    pub fn new(rate: InterestRate<'a, T>) -> Self {
        Self { rate }
    }
}

impl<'a, T> RateCurve for FlatRateCurve<'a, T> 
where
    T: Real + PartialOrd,
    for<'b> &'b T: Add<&'b T, Output = T> + 
                   Sub<&'b T, Output = T> + 
                   Mul<&'b T, Output = T> + 
                   Div<&'b T, Output = T> +
                   Neg<Output = T>,
{
    // 1. You must define the associated type required by the trait
    type T = T;

    fn zero_rate(&self, _t: &T) -> T {
        // 2. Ensure self.rate has a .value field
        self.rate.value.clone()
    }

    fn discount_factor(&self, t: &T) -> T {
        self.rate.discount_factor(t)
    }
}

impl<'a, T: Real + PartialOrd> RateCurve for InterpolatedRateCurve<'a, T> 
where
    for<'b> &'b T: Add<&'b T, Output = T> + 
                   Sub<&'b T, Output = T> + 
                   Mul<&'b T, Output = T> + 
                   Div<&'b T, Output = T> + 
                   Neg<Output = T>,
{
    // 1. Link the associated type to the generic T
    type T = T;

    fn zero_rate(&self, t: &T) -> T {
        self.interpolator.interpolate(t)
    }

    fn discount_factor(&self, t: &T) -> T {
        let r = self.zero_rate(t);
        
        // 2. Wrap the interpolated rate in an InterestRate object
        let rate = InterestRate {
            value: r,
            convention: self.rates[0].convention,
            compounding: self.rates[0].compounding,
            frequency: self.rates[0].frequency,
        };
        
        // 3. Return the calculated discount factor
        rate.discount_factor(t)
    }
}

pub struct InterpolatedRateCurve<'a, T: Real + PartialOrd> {
    #[allow(dead_code)]
    reference_date: NaiveDate,
    #[allow(dead_code)]
    tenors: Vec<Tenor>,
    rates: Vec<InterestRate<'a, T>>,
    interpolator: LinearInterpolator<T>,
}

impl<'a, T: Real + PartialOrd> InterpolatedRateCurve<'a, T> {
    pub fn new(
        reference_date: NaiveDate,
        tenors: Vec<Tenor>,
        rates: Vec<InterestRate<'a, T>>,
        calculator: &dyn PeriodCalculator<'a>,
    ) -> Result<Self, CurveError> {
        if tenors.len() != rates.len() {
            return Err(CurveError::LengthMismatch);
        }

        let year_fractions: Vec<T> = tenors
            .iter()
            .zip(rates.iter())
            .map(|(tenor, rate)| {
                let end_date = tenor.advance(reference_date);
                let yf = calculator.year_fraction(reference_date, end_date, rate.convention).0;
                
                // Lift the f64 into the generic type T
                T::from_f64(yf)
            })
            .collect();

        let rate_values: Vec<T> = rates.iter().map(|r| r.value.clone()).collect();
        let interpolator = LinearInterpolator::new(year_fractions, rate_values)?;

        Ok(Self { reference_date, tenors, rates, interpolator })
    }
}



#[derive(Debug, Clone)]
pub struct ContinuousRateCurve<'a, T: Real> {
    rate: InterestRate<'a, T>,
}

impl<'a, T: Real> ContinuousRateCurve<'a, T> {
    /// Creates a new curve from a raw T value.
    /// Internal InterestRate is set to Continuous compounding.
    pub fn new(value: T) -> Self {
        Self {
            rate: InterestRate {
                value: value, // Automatically converts here
                compounding: Compounding::Continuous,
                frequency: Frequency::Infinite,
                convention: DayCountConvention::Actual365Fixed, 
            },
        }
    }
}

impl<'a, T> RateCurve for ContinuousRateCurve<'a, T>
where
    T: Real + PartialOrd,
    for<'b> &'b T: Add<&'b T, Output = T> + 
                   Sub<&'b T, Output = T> + 
                   Mul<&'b T, Output = T> + 
                   Div<&'b T, Output = T> +
                   Neg<Output = T>,
{
    // The associated type must match the generic used in the struct
    type T = T;

    fn zero_rate(&self, _t: &T) -> T {
        // Accessing the value from the inner rate object
        self.rate.value.clone()
    }

    fn discount_factor(&self, t: &T) -> T {
        // Since compounding is Continuous, this performs: exp(-r * t)
        // Ensure that the 't' passed in matches the curve's expected type
        self.rate.discount_factor(t)
    }
}