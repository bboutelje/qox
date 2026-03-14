use crate::traits::real::Real;

pub trait Payoff<T: Real> 
{
    fn calculate(&self, spot: T) -> T;
}

pub trait InitialConditions<T> {
    fn get_value(self, spot: T) -> T;
}

#[derive(Copy, Clone)]
pub struct PayoffAsInitialConditions<T: Real, P: Payoff<T>>
{
    pub payoff: P,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Real, P: Payoff<T> + Copy> PayoffAsInitialConditions<T, P> {
    pub fn new(payoff: P) -> Self {
        Self {
            payoff,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, P> InitialConditions<T> for PayoffAsInitialConditions<T, P>
where
    T: Real,
    P: Payoff<T> + Copy,
{
    fn get_value(self, spot: T) -> T {
        self.payoff.calculate(spot)
    }
}
