use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{core::period::DayCountConvention, traits::real::Real};

#[derive(Debug, Clone, Copy)]
pub enum Compounding {
    Simple,
    Compounded,
    Continuous,
    SimpleThenCompounded,
}

#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    Annual = 1,
    SemiAnnual = 2,
    Quarterly = 4,
    Monthly = 12,
    Once = 0,
    Infinite = -1,
}

pub trait Discountable<T: Real> {
    fn discount_factor(&self, t: &T) -> T;
}


#[derive(Debug, Clone)]
pub struct InterestRate<'a, T> {
    pub value: T,
    pub convention: DayCountConvention<'a>,
    pub compounding: Compounding,
    pub frequency: Frequency,
}

impl<'a, T: Real + PartialOrd> Discountable<T> for InterestRate<'a, T> 
where
    for<'b> &'b T: Add<&'b T, Output = T> + Sub<&'b T, Output = T> + 
                   Mul<&'b T, Output = T> + Div<&'b T, Output = T> +
                   Neg<Output = T>,
{
    fn discount_factor(&self, t: &T) -> T {
        let one = T::one();
        let r = &self.value; // Borrow the rate once

        match self.compounding {
            Compounding::Simple => {
                // df = 1 / (1 + r * t)
                &one / &(&one + &k_mul(r, t))
            }

            Compounding::Continuous => {
                // df = exp(-(r * t))
                let rt = r * t; // Multiplying &T * &T
                (-&rt).exp()
            }

            Compounding::Compounded => {
                let f = T::from_f64(self.frequency as i32 as f64);
                // df = 1 / (1 + r/f)^(f*t)
                let base = &one + &(r / &f);
                let exponent = &f * t;
                &one / &base.powf(&exponent)
            }

            Compounding::SimpleThenCompounded => {
                if t <= &one {
                    &one / &(&one + &k_mul(r, t))
                } else {
                    let f = T::from_f64(self.frequency as i32 as f64);
                    let base = &one + &(r / &f);
                    let exponent = &f * t;
                    &one / &base.powf(&exponent)
                }
            }
        }
    }
}

// Small helper to avoid syntax clutter for (r * t)
fn k_mul<T>(a: &T, b: &T) -> T 
where for<'a> &'a T: Mul<&'a T, Output = T> {
    a * b
}

impl<'a> InterestRate<'a, f64> {
    pub fn implied_rate(
        df: f64,
        t: f64,
        compounding: Compounding,
        frequency: Frequency,
        convention: DayCountConvention<'a>,
    ) -> Self {
        let value = match compounding {
            Compounding::Simple => (1.0 / df - 1.0) / t,

            Compounding::Continuous => -df.ln() / t,

            Compounding::Compounded => {
                let f = frequency as i32 as f64;
                f * ((1.0 / df).powf(1.0 / (f * t)) - 1.0)
            }

            Compounding::SimpleThenCompounded => {
                if t <= 1.0 {
                    (1.0 / df - 1.0) / t
                } else {
                    let f = frequency as i32 as f64;
                    f * ((1.0 / df).powf(1.0 / (f * t)) - 1.0)
                }
            }
        };

        InterestRate {
            value,
            convention,
            compounding,
            frequency,
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct ContinuousInterestRate<'a, T> {
//     pub value: T,
//     pub convention: DayCountConvention<'a>,
// }

// impl<'a> Discountable<f64> for ContinuousInterestRate<'a, f64> {
//     /// Formula: DF = e^(-r * t)
//     fn discount_factor(&self, t: f64) -> f64 {
//         (-self.value * t).exp()
//     }
// }

// impl<'a> ContinuousInterestRate<'a, f64> {
//     pub fn new(value: f64, convention: DayCountConvention<'a>) -> Self {
//         Self { value, convention }
//     }

//     /// Calculates the continuously compounded rate from a discount factor
//     pub fn from_discount_factor(
//         df: f64, 
//         t: f64, 
//         convention: DayCountConvention<'a>
//     ) -> Self {
//         let value = -df.ln() / t;
//         Self { value, convention }
//     }

//     /// Helper to convert this into a standard InterestRate struct
//     pub fn to_interest_rate(&self) -> InterestRate<'a, f64> {
//         InterestRate {
//             value: self.value,
//             convention: self.convention.clone(),
//             compounding: Compounding::Continuous,
//             frequency: Frequency::Once, // Frequency is ignored in continuous compounding
//         }
//     }
// }