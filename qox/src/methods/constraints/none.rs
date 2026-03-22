use crate::{
    methods::{constraints::Constraint, finite_difference::meshers::SpatialGrid},
    types::Real,
};

#[derive(Clone, Copy)]
pub struct NoConstraint;
impl NoConstraint {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Real, SG: SpatialGrid<T>> Constraint<T, SG> for NoConstraint {
    #[inline(always)]
    fn apply(&self, _: &mut [T], _: &SG) {}

    fn lower_bound(&self, _i: usize, _mesher: &SG) -> T {
        todo!()
    }
}
