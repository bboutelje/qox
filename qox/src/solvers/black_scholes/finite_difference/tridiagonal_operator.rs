use crate::traits::{linear_operator::LinearOperator, real::Real};
use std::cell::RefCell;

// Helper to store the factorization results
struct TridiagonalCache<T> {
    coeff: T,
    c_prime: Vec<T>,
    m_inv: Vec<T>,
}

pub struct TridiagonalOperator<T> where T: Real {
    pub lower: Vec<T>,
    pub diag: Vec<T>,
    pub upper: Vec<T>,
    cache: RefCell<Option<TridiagonalCache<T>>>,
}

impl<T> TridiagonalOperator<T> 
where T: Real 
{
    pub fn new(lower: Vec<T>, diag: Vec<T>, upper: Vec<T>) -> Self {
        let n = diag.len();
        if n < 3 {
            panic!("TridiagonalOperator requires at least 3 elements, found {}", n);
        }
        
        TridiagonalOperator {
            lower,
            diag,
            upper,
            cache: RefCell::new(None),
        }
    }
}

impl<T> LinearOperator<T> for TridiagonalOperator<T> 
where T: Real 
{
    fn size(&self) -> usize {
        self.diag.len()
    }

    fn apply_into(&self, v: &[T], _t: T, out: &mut [T]) {
        let n = self.size();
        for i in 0..n {
            let mut val = self.diag[i] * v[i];
            if i > 0 { val += self.lower[i] * v[i - 1]; }
            if i < n - 1 { val += self.upper[i] * v[i + 1]; }
            out[i] = val;
        }
    }

    
    fn setup_coeff(&self, coeff: T) {
        let mut cache = self.cache.borrow_mut();
        
        // Skip if already computed for this coeff
        if let Some(ref c) = *cache {
            if (c.coeff - coeff).abs() < T::from_f64(1e-12) { return; }
        }

        let n = self.size();
        let mut c_prime = vec![T::zero(); n];
        let mut m_inv = vec![T::zero(); n];

        // Solve (I - coeff * L)x = b
        // Diagonal: d_i' = 1 - coeff * diag[i]
        // Off-diagonals: a_i = -coeff * lower[i], c_i = -coeff * upper[i]
        
        

        let d0 = T::one() - coeff * self.diag[0];
        m_inv[0] = T::one() / d0;
        c_prime[0] = (-coeff * self.upper[0]) * m_inv[0];

        for i in 1..n {
            let a = -coeff * self.lower[i];
            let d = T::one() - coeff * self.diag[i];
            let c = if i < n - 1 { -coeff * self.upper[i] } else { T::zero() };
            
            let m = d - a * c_prime[i - 1];
            m_inv[i] = T::one() / m;
            c_prime[i] = c * m_inv[i];
        }

        *cache = Some(TridiagonalCache { coeff, c_prime, m_inv });
    }

    fn solve_inverse_into(&self, b: &[T], _coeff: T, _t: T, dest: &mut [T], z_buffer: &mut [T]) {
        let cache = self.cache.borrow();
        let c = cache.as_ref().expect("TridiagonalOperator: setup_coeff must be called first");
        let n = self.size();

        // Forward sweep: Use the precomputed m_inv and the modified 'a' 
        // (which is implicitly handled by using the cache's c_prime)
        z_buffer[0] = b[0] * c.m_inv[0];
        for i in 1..n {
            // The Thomas algorithm forward step:
            // z_i = (b_i - a_i * z_{i-1}) / m_i
            // where a_i = -coeff * lower[i]
            let a_scaled = -c.coeff * self.lower[i];
            z_buffer[i] = (b[i] - a_scaled * z_buffer[i - 1]) * c.m_inv[i];
        }

        // Back substitution: Use the precomputed c_prime
        dest[n - 1] = z_buffer[n - 1];
        for i in (0..n - 1).rev() {
            dest[i] = z_buffer[i] - c.c_prime[i] * dest[i + 1];
        }
    }

}