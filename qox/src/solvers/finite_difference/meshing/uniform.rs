use std::ops::{Add, Div, Mul, Sub};

use crate::traits::{fdm_1d_mesher::Mesher1d, real::Real};

pub struct UniformMesher1d<T: Real> {
    centers: Vec<T>,
    h_plus: Vec<T>,
    h_minus: Vec<T>,
}

impl<T: Real> UniformMesher1d<T>
where
    for<'a> &'a T: Add<&'a T, Output = T> + 
                   Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + 
                   Div<&'a T, Output = T> 
{
    pub fn new(start: T, end: T, size: usize) -> Self {

        let mut centers = Vec::with_capacity(size);
        let n_minus_1 = T::from_f64((size - 1) as f64);
        let dx = &(&end - &start) / &n_minus_1;

        for i in 0..size {
            let i_t = T::from_f64(i as f64);
            let s_i = &start + &(&i_t * &dx);
            centers.push(s_i);
        }

        let (h_plus, h_minus) = Self::build_distances(&centers);
        Self { centers, h_plus, h_minus }
    }

    fn build_distances(centers: &[T]) -> (Vec<T>, Vec<T>) {
        let n = centers.len();
        let mut hp = vec![T::zero(); n];
        let mut hm = vec![T::zero(); n];
        for (i, window) in centers.windows(2).enumerate() {
            let (left, right) = (&window[0], &window[1]);
            let diff = right - left;
            
            hp[i] = diff.clone();
            hm[i + 1] = diff;
        }
        (hp, hm)
    }
}

impl<T: Real> Mesher1d<T> for UniformMesher1d<T>
where 
        for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                    Mul<&'a T, Output = T> + Div<&'a T, Output = T> {
    fn size(&self) -> usize { self.centers.len() }
    fn centers(&self) -> &[T] { &self.centers }
    fn h_plus(&self) -> &[T] { &self.h_plus }
    fn h_minus(&self) -> &[T] { &self.h_minus }
    
    
}