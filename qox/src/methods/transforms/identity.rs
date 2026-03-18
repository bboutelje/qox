use std::marker::PhantomData;

use crate::{methods::transforms::Transform, types::Real};

#[derive(Copy, Clone)]
pub struct IdentityTransform<T> {
    _marker: PhantomData<T>,
}

impl<T: Real> Transform<T> for IdentityTransform<T> {
    fn to_physical(&self, x: T) -> T {
        x
    }
    fn to_transform(&self, s: T) -> T {
        s
    }
    fn jacobian(&self, _: T) -> T {
        T::one()
    }
    fn hessian(&self, _: T) -> T {
        T::zero()
    }
}
