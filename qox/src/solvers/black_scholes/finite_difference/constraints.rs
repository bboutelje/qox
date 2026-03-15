use crate::traits::{
    constraint::Constraint, fdm_mesher::Mesher1d, payoff::InitialConditions, real::Real,
};

#[derive(Clone, Copy)]
pub struct NoConstraint;
impl NoConstraint {
    pub fn new() -> Self {
        Self
    }
}
impl<T: Real, M: Mesher1d<T>> Constraint<T, M> for NoConstraint {
    #[inline(always)]
    fn apply(&self, _: &mut [T], _: &M) {}

    fn lower_bound(&self, _i: usize, _mesher: &M) -> T {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct AmericanConstraint<IC> {
    pub payoff: IC,
}
impl<IC> AmericanConstraint<IC> {
    pub fn new(payoff: IC) -> Self {
        Self { payoff }
    }
}
impl<T: Real, IC: InitialConditions<T> + Copy, M: Mesher1d<T>> Constraint<T, M>
    for AmericanConstraint<IC>
{
    #[inline(always)]
    fn apply(&self, price: &mut [T], mesher: &M) {
        for i in 0..price.len() {
            let p = self.payoff.get_value(mesher.location(i));
            if price[i] < p {
                price[i] = p;
            }
        }
    }

    fn lower_bound(&self, i: usize, mesher: &M) -> T {
        self.payoff.get_value(mesher.location(i))
    }
}
