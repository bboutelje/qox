// solvers/fdm/operators.rs

use std::ops::{Add, Div, Mul, Sub};

use crate::traits::real::Real;

pub struct TridiagonalCoeffs<T: Real> {
    pub a: Vec<T>, // Lower diagonal
    pub b: Vec<T>, // Main diagonal
    pub c: Vec<T>, // Upper diagonal
}

impl<T> TridiagonalCoeffs<T>
    where T: Real,
    for<'a> &'a T: Mul<&'a T, Output = T> + Div<&'a T, Output = T> + 
    Add<&'a T, Output = T> + Sub<&'a T, Output = T> {
    pub fn black_scholes_operator(
        grid: &[T], 
        r: &T, 
        sigma: &T, 
        dt: &T
    ) -> Self {
        let n = grid.len();
        let mut a = vec![T::zero(); n];
        let mut b = vec![T::one(); n]; // Default to 1 for Identity at boundaries
        let mut c = vec![T::zero(); n];

        let vol_sq = sigma * sigma;
        let half_vol_sq = &T::from_f64(0.5) * &vol_sq;

        for i in 1..n-1 {
            let s = &grid[i];
            let h_minus = s - &grid[i-1];
            let h_plus = &grid[i+1] - s;
            
            // Discretization coefficients for non-uniform grid
            // First Derivative (D1)
            let d1_l = -(&h_plus / &(&h_minus * &(&h_minus + &h_plus)));
            let d1_m = (&h_plus - &h_minus) / &(&h_minus * &h_plus);
            let d1_u = &h_minus / &(&h_plus * &(&h_minus + &h_plus));

            // Second Derivative (D2)
            let d2_l = T::from_f64(2.0) / &(&h_minus * &(&h_minus + &h_plus));
            let d2_m = -T::from_f64(2.0) / &(&h_minus * &h_plus);
            let d2_u = T::from_f64(2.0) / &(&h_plus * &(&h_minus + &h_plus));

            let drift = r * s;
            let diffusion = &half_vol_sq * &(s * s);

            // Operator L = diffusion*D2 + drift*D1 - r*I
            let l_row_i = &(&diffusion * &d2_l) + &(&drift * &d1_l);
            let d_row_i = &(&(&diffusion * &d2_m) + &(&drift * &d1_m)) - r;
            let u_row_i = &(&diffusion * &d2_u) + &(&drift * &d1_u);

            // Implicit Euler: b - dt*L
            a[i] = -(dt * &l_row_i);
            b[i] = T::one() - &(dt * &d_row_i);
            c[i] = -(dt * &u_row_i);
        }
        
        Self { a, b, c }
    }
}


pub fn solve_tridiagonal<T: Real>(a: &[T], b: &[T], c: &[T], r: &[T]) -> Vec<T>
where 
    T: Real,
    for<'a> &'a T: Div<&'a T, Output = T> + Mul<&'a T, Output = T> + Sub<&'a T, Output = T>
{
    let n = r.len();
    let mut c_prime = vec![T::zero(); n];
    let mut d_prime = vec![T::zero(); n];
    let mut res = vec![T::zero(); n];

    // Forward sweep
    c_prime[0] = &c[0] / &b[0];
    d_prime[0] = &r[0] / &b[0];

    for i in 1..n {
        let m = &b[i] - &(&a[i] * &c_prime[i-1]);
        if i < n - 1 {
            c_prime[i] = &c[i] / &m;
        }
        d_prime[i] = &(&r[i] - &(&a[i] * &d_prime[i-1])) / &m;
    }

    // Back substitution
    res[n-1] = d_prime[n-1].clone();
    for i in (0..n-1).rev() {
        res[i] = &d_prime[i] - &(&c_prime[i] * &res[i+1]);
    }
    res
}

