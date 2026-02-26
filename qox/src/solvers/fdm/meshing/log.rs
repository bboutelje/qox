use std::ops::{Add, Div, Mul, Sub};
use crate::traits::{fdm_1d_mesher::Fdm1dMesher, real::Real};


pub struct FdmLog1dMesher<T: Real, M: Fdm1dMesher<T>>
where 
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                    Mul<&'a T, Output = T> + Div<&'a T, Output = T> {
    underlying: M,
    exp_centers: Vec<T>,
}

impl<T: Real, M: Fdm1dMesher<T>> FdmLog1dMesher<T, M> 
where
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + Div<&'a T, Output = T> 
{
    pub fn new(mesher: M) -> Self {
        let exp_centers = mesher.centers()
            .iter()
            .map(|x| x.exp())
            .collect();

        Self {
            underlying: mesher,
            exp_centers,
        }
    }
}

impl<T: Real, M: Fdm1dMesher<T>> Fdm1dMesher<T> for FdmLog1dMesher<T, M> 
where
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + Div<&'a T, Output = T> 
{
    fn size(&self) -> usize { self.underlying.size() }
    fn centers(&self) -> &[T] { self.underlying.centers() }
    fn h_plus(&self) -> &[T] { self.underlying.h_plus() }
    fn h_minus(&self) -> &[T] { self.underlying.h_minus() }

    fn location(&self, index: usize) -> &T {
        &self.exp_centers[index]
    }
}