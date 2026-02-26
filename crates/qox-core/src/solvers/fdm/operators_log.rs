use std::ops::{Add, Div, Mul, Sub};
use crate::traits::real::Real;
use crate::traits::fdm_1d_mesher::Fdm1dMesher;

pub struct TridiagonalCoeffs<T: Real> {
    pub a: Vec<T>, // Lower diagonal
    pub b: Vec<T>, // Main diagonal
    pub c: Vec<T>, // Upper diagonal
}

impl<T> TridiagonalCoeffs<T>
where 
    T: Real,
    for<'a> &'a T: Mul<&'a T, Output = T> + Div<&'a T, Output = T> + 
                   Add<&'a T, Output = T> + Sub<&'a T, Output = T> +
                   std::ops::Neg<Output = T> 
{
    /// Builds the Black-Scholes operator in Log-Space.
    /// 
    /// The PDE being solved is:
    /// dV/dt + 0.5*sigma^2 * d2V/dx2 + (r - 0.5*sigma^2) * dV/dx - rV = 0
    pub fn black_scholes_log_operator<M: Fdm1dMesher<T>>(
        mesher: &M, 
        r: &T, 
        sigma: &T, 
        dt: &T
    ) -> Self {
        let n = mesher.size();
        let mut a = vec![T::zero(); n];
        let mut b = vec![T::one(); n];  // Identity at boundaries
        let mut c = vec![T::zero(); n];

        // 1. Pre-calculate constant coefficients for log-space
        let vol_sq = sigma * sigma;
        let half_vol_sq = &T::from_f64(0.5) * &vol_sq;
        
        // Log-drift: (r - 0.5 * sigma^2)
        let drift = r - &half_vol_sq;
        let diffusion = &half_vol_sq;
        let neg_dt = &-dt;

        // 2. Fetch mesher stencils
        let hp = mesher.h_plus();
        let hm = mesher.h_minus();

        for i in 1..n-1 {
            let h_p = &hp[i];
            let h_m = &hm[i];
            let h_sum = h_p + h_m;

            // Discretization for First Derivative (D1)
            let d1_l = &(-h_p) / &(h_m * &h_sum);
            let d1_m = &(h_p - h_m) / &(h_m * h_p);
            let d1_u = h_m / &(h_p * &h_sum);

            // Discretization for Second Derivative (D2)
            let d2_l = T::from_f64(2.0) / &(h_m * &h_sum);
            let d2_m = &T::from_f64(-2.0) / &(h_m * h_p);
            let d2_u = T::from_f64(2.0) / &(h_p * &h_sum);

            // Local Operator L = diffusion*D2 + drift*D1 - r*I
            let l_row = &(diffusion * &d2_l) + &(&drift * &d1_l);
            let d_row = &(&(diffusion * &d2_m) + &(&drift * &d1_m)) - r;
            let u_row = &(diffusion * &d2_u) + &(&drift * &d1_u);

            // Implicit Euler Step: (I - dt*L)V_new = V_old
            a[i] = neg_dt * &l_row;
            b[i] = T::one() + &(neg_dt * &d_row);
            c[i] = neg_dt * &u_row;
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
    if n == 0 { return Vec::new(); }
    
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