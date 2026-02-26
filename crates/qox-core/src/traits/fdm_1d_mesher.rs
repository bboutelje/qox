use crate::traits::real::Real;
use std::ops::{Add, Div, Mul, Sub};

pub trait Fdm1dMesher<T: Real>
where 
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                    Mul<&'a T, Output = T> + Div<&'a T, Output = T> {
    fn size(&self) -> usize;
    fn centers(&self) -> &[T];
    fn h_plus(&self) -> &[T];
    fn h_minus(&self) -> &[T];
    
    fn location(&self, index: usize) -> &T {
        &self.centers()[index]   
    }
}