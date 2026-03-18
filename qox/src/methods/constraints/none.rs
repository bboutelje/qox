use crate::{
    methods::{constraints::Constraint, finite_difference::meshers::Mesher1d},
    types::Real,
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
