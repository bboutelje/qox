
use crate::traits::{instrument::{OptionInstrument, OptionType}, rate_curve::RateCurve, real::Real, vol_surface::VolSurface};

pub trait Payoff<T: Real> 
{
    fn calculate(&self, spot: T) -> T;
}

pub trait InitialCondition<T> {
    fn get_value(self, spot: T) -> T;
}

#[derive(Copy, Clone)]
pub struct PayoffAsInitialCondition<T: Real, P: Payoff<T>>
{
    pub payoff: P,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Real, P: Payoff<T> + Copy> PayoffAsInitialCondition<T, P> {
    pub fn new(payoff: P) -> Self {
        Self {
            payoff,
            _marker: std::marker::PhantomData,
        }
    }
}

// You must explicitly add the + Real bound to P here
impl<T, P> InitialCondition<T> for PayoffAsInitialCondition<T, P>
where
    T: Real,
    P: Payoff<T> + Copy,
{
    fn get_value(self, spot: T) -> T {
        self.payoff.calculate(spot)
    }
}
// impl<T, I> InitialCondition<T> for &I
// where 
//     T: Real,
//     I: OptionInstrument<T>,
// {
//     fn get_value(self, spot: T) -> T {
//         let strike = T::from_f64(&self.strike());
//         match self.option_type() {
//             OptionType::Call => (spot - strike).max(T::zero()),
//             OptionType::Put => (strike - spot).max(T::zero()),
//         }
//     }
// }