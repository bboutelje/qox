pub mod evaluator;
pub mod evaluator_kernel;

use crate::traits::{instrument::OptionType, payoff::Payoff, real::Real};

#[derive(Copy, Clone)]
pub struct VanillaPayoff {
    pub strike: f64,
    pub option_type: OptionType,
}

impl<T:Real> Payoff<T> for VanillaPayoff {
    fn calculate(&self, spot: T) -> T
    {
        let k = T::from_f64(self.strike);
        match self.option_type {
            OptionType::Call => {
                if spot > k {
                    spot - k
                } else {
                    T::zero()
                }
            }
            OptionType::Put => {
                if k > spot {
                    k - spot
                } else {
                    T::zero()
                }
            }
        }
    }
}