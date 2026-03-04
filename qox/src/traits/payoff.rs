
use crate::traits::{instrument::OptionInstrument, real::Real};

pub trait Payoff 
{
    fn calculate<T: Real>(&self, spot: T) -> T;
}

pub trait InitialCondition<T> {
    fn get_value(self, spot: T) -> T;
}

impl<I, T> InitialCondition<T> for I 
where 
    I: OptionInstrument,
    T: Real
{
    fn get_value(self, spot: T) -> T {
        let strike = T::from_f64(self.strike());
        if self.is_call() {
            (spot - strike).max(T::zero())
        } else {
            (strike - spot).max(T::zero())
        }
    }
}