use std::ops::{Add, Div, Mul, Sub};

use crate::{core::error::InterpolationError, traits::real::Real};

pub trait Interpolator1D<T> {
    fn interpolate(&self, x: &T) -> T;
}

pub trait Interpolator2D<T> {
    fn interpolate(&self, x: &T, y: &T) -> T;
}

#[derive(Clone, Debug)]
pub struct LinearInterpolator<T: Real> {
    x: Vec<T>,
    y: Vec<T>,
}

impl<T: Real + PartialOrd> LinearInterpolator<T> {
    pub fn new(x: Vec<T>, y: Vec<T>) -> Result<Self, InterpolationError> {
        if x.len() != y.len() {
            return Err(InterpolationError::LengthMismatch);
        }
        if x.len() < 2 {
            return Err(InterpolationError::InsufficientPoints);
        }
        
        // Check x is sorted
        for i in 1..x.len() {
            if x[i] <= x[i-1] {
                return Err(InterpolationError::NotMonotonic);
            }
        }
        
        Ok(Self { x, y })
    }
    
    fn find_interval(&self, x: &T) -> usize {
        // Binary search for the interval
        if x <= &self.x[0] {
            return 0;
        }
        if x >= &self.x[self.x.len() - 1] {
            return self.x.len() - 2;
        }
        
        let mut left = 0;
        let mut right = self.x.len() - 1;
        
        while right - left > 1 {
            let mid = (left + right) / 2;
            if self.x[mid] <= *x {
                left = mid;
            } else {
                right = mid;
            }
        }
        
        left
    }
}

impl<T: Real + PartialOrd> Interpolator1D<T> for LinearInterpolator<T> 
where
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + Div<&'a T, Output = T>,
{
    fn interpolate(&self, x: &T) -> T {
        let i = self.find_interval(x); // Pass x as reference
        
        // 1. Get references to the points
        let x0 = &self.x[i];
        let x1 = &self.x[i + 1];
        let y0 = &self.y[i];
        let y1 = &self.y[i + 1];
        
        // 2. Math using references (&T - &T)
        // Note: x is owned here, so we borrow it as &x
        let slope = &(y1 - y0) / &(x1 - x0);
        let dx = x - x0;
        
        // y0 + slope * dx
        y0 + &(&slope * &dx)
    }
}

#[derive(Clone, Debug)]
pub struct BilinearInterpolator<T: Real> {
    x: Vec<T>,
    y: Vec<T>,
    z: Vec<Vec<T>>, // z[i][j] corresponds to (x[i], y[j])
}

impl<T: Real + PartialOrd> BilinearInterpolator<T> {
    pub fn new(x: Vec<T>, y: Vec<T>, z: Vec<Vec<T>>) -> Result<Self, String> {
        // 1. Dimension Validation
        if z.len() != x.len() {
            return Err("z outer dimension must match x length".to_string());
        }
        for row in &z {
            if row.len() != y.len() {
                return Err("z inner dimension must match y length".to_string());
            }
        }
        if x.len() < 2 || y.len() < 2 {
            return Err("Need at least 2 points in each dimension".to_string());
        }

        // 2. Sorting Validation using windows() 
        // We compare references (&T <= &T), which works without Copy
        if x.windows(2).any(|w| w[1] <= w[0]) {
            return Err("x values must be strictly increasing".to_string());
        }
        if y.windows(2).any(|w| w[1] <= w[0]) {
            return Err("y values must be strictly increasing".to_string());
        }

        Ok(Self { x, y, z })
    }

    fn find_interval(values: &[T], v: &T) -> usize {
        let n = values.len();
        
        // Boundary checks using references
        if v <= &values[0] {
            return 0;
        }
        if v >= &values[n - 1] {
            return n - 2;
        }

        // 3. Binary Search
        // Note: we compare values[mid] (a reference) with v (a reference)
        let mut left = 0;
        let mut right = n - 1;

        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if &values[mid] <= v {
                left = mid;
            } else {
                right = mid;
            }
        }

        left
    }
}

impl<T: Real + PartialOrd> Interpolator2D<T> for BilinearInterpolator<T> 
where
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + 
                   Mul<&'a T, Output = T> + Div<&'a T, Output = T>,
{
    fn interpolate(&self, x: &T, y: &T) -> T {
        // 1. Find indices (pass references, no deref *)
        let i = Self::find_interval(&self.x, x);
        let j = Self::find_interval(&self.y, y);
        
        // 2. Grid references
        let x0 = &self.x[i];
        let x1 = &self.x[i + 1];
        let y0 = &self.y[j];
        let y1 = &self.y[j + 1];
        
        let z00 = &self.z[i][j];
        let z01 = &self.z[i][j + 1];
        let z10 = &self.z[i + 1][j];
        let z11 = &self.z[i + 1][j + 1];

        let one = T::from_f64(1.0);

        // 3. Normalized weights (tx, ty)
        let tx = &(x - x0) / &(x1 - x0);
        let ty = &(y - y0) / &(y1 - y0);
        
        // 4. Interpolate in X-direction
        // z0 = z00 * (1 - tx) + z10 * tx
        let one_minus_tx = &one - &tx;
        let z0 = &(z00 * &one_minus_tx) + &(z10 * &tx);
        
        // z1 = z01 * (1 - tx) + z11 * tx
        let z1 = &(z01 * &one_minus_tx) + &(z11 * &tx);
        
        // 5. Interpolate in Y-direction
        // result = z0 * (1 - ty) + z1 * ty
        let one_minus_ty = &one - &ty;
        &(&z0 * &one_minus_ty) + &(&z1 * &ty)
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_linear_interpolation() {
//         // Explicitly use f64 to satisfy the <T: Real> bound
//         let x: Vec<f64> = vec![0.0, 1.0, 2.0];
//         let y: Vec<f64> = vec![0.0, 10.0, 20.0];
        
//         let interp = LinearInterpolator::new(x, y).unwrap();
        
//         // We use a small epsilon for float comparison, though f64 
//         // linear math is usually exact for these simple values.
//         assert!((interp.interpolate(0.0) - 0.0).abs() < 1e-10);
//         assert!((interp.interpolate(0.5) - 5.0).abs() < 1e-10);
//         assert!((interp.interpolate(1.0) - 10.0).abs() < 1e-10);
//         assert!((interp.interpolate(1.5) - 15.0).abs() < 1e-10);
//         assert!((interp.interpolate(2.0) - 20.0).abs() < 1e-10);
//     }
    
//     #[test]
//     fn test_bilinear_interpolation() {
//         let x: Vec<f64> = vec![0.0, 1.0];
//         let y: Vec<f64> = vec![0.0, 1.0];
//         let z: Vec<Vec<f64>> = vec![
//             vec![0.0, 1.0], // z(x0, y0), z(x0, y1)
//             vec![2.0, 3.0]  // z(x1, y0), z(x1, y1)
//         ];
        
//         let interp = BilinearInterpolator::new(x, y, z).unwrap();
        
//         assert!((interp.interpolate(0.0, 0.0) - 0.0).abs() < 1e-10);
//         assert!((interp.interpolate(1.0, 1.0) - 3.0).abs() < 1e-10);
//         assert!((interp.interpolate(0.5, 0.5) - 1.5).abs() < 1e-10);
//     }
// }