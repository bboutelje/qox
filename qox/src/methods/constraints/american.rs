use crate::{
    methods::{constraints::Constraint, finite_difference::meshers::Mesher1d},
    traits::payoff::InitialConditions,
    types::Real,
};

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
            price[i] = price[i].max(p);
        }
    }

    #[inline(always)]
    fn lower_bound(&self, i: usize, mesher: &M) -> T {
        self.payoff.get_value(mesher.location(i))
    }
}
