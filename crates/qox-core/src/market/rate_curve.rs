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

impl<'a, T: Real + PartialOrd> RateCurve<T> for FlatRateCurve<'a, T> 
where
    for<'b> &'b T: Add<&'b T, Output = T> + 
                   Sub<&'b T, Output = T> + 
                   Mul<&'b T, Output = T> + 
                   Div<&'b T, Output = T> +
                   Neg<Output = T>,
{
    fn zero_rate(&self, _t: &T) -> T {
        // Now 'rate' is being read here
        self.rate.value.clone()
    }

    fn discount_factor(&self, t: &T) -> T {
        // And here
        self.rate.discount_factor(t)
    }
}

impl<'a, T: Real + PartialOrd> RateCurve<T> for InterpolatedRateCurve<'a, T> 
where
    for<'b> &'b T: Add<&'b T, Output = T> + 
                   Sub<&'b T, Output = T> + 
                   Mul<&'b T, Output = T> + 
                   Div<&'b T, Output = T> + 
                   Neg<Output = T>,
{
    fn zero_rate(&self, t: &T) -> T {
        self.interpolator.interpolate(t)
    }

    fn discount_factor(&self, t: &T) -> T {
        let r = self.zero_rate(t);
        let rate = InterestRate {
            value: r,
            convention: self.rates[0].convention,
            compounding: self.rates[0].compounding,
            frequency: self.rates[0].frequency,
        };
        
        // Now the compiler knows that for this T, 
        // InterestRate correctly implements Discountable.
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
    pub fn new<I: Into<T>>(value: I) -> Self {
        Self {
            rate: InterestRate {
                value: value.into(), // Automatically converts here
                compounding: Compounding::Continuous,
                frequency: Frequency::Infinite,
                convention: DayCountConvention::Actual365Fixed, 
            },
        }
    }
}

impl<'a, T: Real + PartialOrd> RateCurve<T> for ContinuousRateCurve<'a, T>
where
    for<'b> &'b T: Add<&'b T, Output = T> + Sub<&'b T, Output = T> + 
                    Mul<&'b T, Output = T> + Div<&'b T, Output = T> +
                    Neg<Output = T>,
{
    fn zero_rate(&self, _t: &T) -> T {
        self.rate.value.clone()
    }

    fn discount_factor(&self, t: &T) -> T {
        // Since compounding is Continuous, this performs: exp(-r * t)
        self.rate.discount_factor(t)
    }
}
