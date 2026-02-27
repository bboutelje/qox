use crate::traits::real::Real;
use std::ops::{Add, Sub, Mul, Div};

pub struct FdmGrid<T: Real> {
    pub centers: Vec<T>,    // The actual nodes S_i
    pub h_plus: Vec<T>,     // Distance to right neighbor: S_{i+1} - S_i
    pub h_minus: Vec<T>,    // Distance to left neighbor: S_i - S_{i-1}
}

impl<T: Real> FdmGrid<T> 
where 
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + Div<&'a T, Output = T> +
                   std::ops::Neg<Output = T>,
{

    pub fn new_linear_space(s_min: T, s_max: T, nodes: usize) -> Self {
        let mut s_centers = Vec::with_capacity(nodes);
        
        // Calculate distance between points: ds = (S_max - S_min) / (nodes - 1)
        let ds = &(&s_max - &s_min) / &T::from_f64((nodes - 1) as f64);

        for i in 0..nodes {
            // S_i = S_min + i * ds
            let s_i = &s_min + &(&T::from_f64(i as f64) * &ds);
            s_centers.push(s_i);
        }

        Self::build_from_centers(s_centers)
    }

    pub fn new_log_space(s_min: T, s_max: T, nodes: usize) -> Self {
        let x_min = s_min.ln();
        let x_max = s_max.ln();
        
        let mut x_centers = Vec::with_capacity(nodes);
        let dx = &(&x_max - &x_min) / &T::from_f64((nodes - 1) as f64);

        for i in 0..nodes {
            x_centers.push(&x_min + &(&T::from_f64(i as f64) * &dx));
        }

        // We still call this, but 'centers' are now ln(S)
        Self::build_from_centers(x_centers)
    }

    fn build_from_centers(centers: Vec<T>) -> Self {
        let n = centers.len();
        let mut h_plus = vec![T::zero(); n];
        let mut h_minus = vec![T::zero(); n];

        for i in 1..n-1 {
            h_plus[i] = &centers[i+1] - &centers[i];
            h_minus[i] = &centers[i] - &centers[i-1];
        }

        Self { centers, h_plus, h_minus }
    }

    pub fn second_derivative_coeffs(&self) -> (Vec<T>, Vec<T>, Vec<T>) {
        let n = self.centers.len();
        let mut l = vec![T::zero(); n];
        let mut d = vec![T::zero(); n];
        let mut u = vec![T::zero(); n];

        let two = T::from_f64(2.0);

        for i in 1..n-1 {
            let hp = &self.h_plus[i];
            let hm = &self.h_minus[i];
            let sum_h = hp + hm;

            l[i] = &two / &(hm * &sum_h);
            d[i] = -&two / &(hp * hm);
            u[i] = &two / &(hp * &sum_h);
        }
        (l, d, u)
    }

    /// Returns the tridiagonal coefficients for the first derivative (Central).
    pub fn first_derivative_coeffs(&self) -> (Vec<T>, Vec<T>, Vec<T>)
    {
        let n = self.centers.len();
        let mut l = vec![T::zero(); n];
        let mut d = vec![T::zero(); n];
        let mut u = vec![T::zero(); n];

        for i in 1..n-1 {
            let hp = &self.h_plus[i];
            let hm = &self.h_minus[i];
            let sum_h = hp + hm;

            // Non-uniform central difference weights
            l[i] = &(&T::zero() - hp) / &(hm * &sum_h); // Note the & before T::zero()
            d[i] = &(hp - hm) / &(hp * hm);
            u[i] = hm / &(hp * &sum_h);
        }
        (l, d, u)
    }
}