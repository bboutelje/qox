use crate::traits::{real::Real, transform::Transform};

#[derive(Copy, Clone)]
pub struct LogTransform<T: Real> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Real> LogTransform<T> {
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }
}

impl<T: Real> Transform<T> for LogTransform<T> {
    fn to_mesh(&self, physical: T) -> T {
        physical.ln()
    }

    fn to_physical(&self, mesh_node: T) -> T {
        mesh_node.exp()
    }

    fn jacobian(&self, xi: T) -> T {
        xi.exp()
    }

    fn hessian(&self, xi: T) -> T {
        xi.exp()
    }
}