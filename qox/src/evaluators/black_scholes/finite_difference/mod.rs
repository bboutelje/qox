pub mod evaluator;
pub mod evaluator_kernel;

use crate::{instruments::future_option::OptionType, traits::{payoff::Payoff, real::Real}};

pub struct VanillaPayoff {
    pub strike: f64,
    pub option_type: OptionType,
}

impl Payoff for VanillaPayoff {
    fn calculate<T:Real>(&self, spot: T) -> T
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