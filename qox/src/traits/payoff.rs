use std::ops::Sub;

use crate::traits::{instrument::OptionInstrument, real::Real};

pub trait Payoff 

{
    fn calculate<T: Real>(&self, spot: &T) -> T
    where
        T: Real + PartialOrd,
        for<'a> &'a T: Sub<&'a T, Output = T>;
}

pub trait InitialCondition<T> {
    fn get_value(&self, spot: &T) -> T;
}

impl<'a, I, T> InitialCondition<T> for &'a I 
where 
    I: OptionInstrument,
    T: Real + PartialOrd,
    for<'b> &'b T: Sub<&'b T, Output = T>
{
    fn get_value(&self, spot: &T) -> T {
        let strike = T::from_f64(self.strike());
        if self.is_call() {
            (spot - &strike).max(&T::zero())
        } else {
            (&strike - spot).max(&T::zero())
        }
    }
}