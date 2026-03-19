use crate::{
    methods::{
        constraints::Constraint, finite_difference::meshers::Mesher1d,
        linear_operators_old::LinearOperator,
    },
    types::Real,
};
use std::{cell::RefCell, marker::PhantomData};

// Helper to store the factorization results
struct TridiagonalCache<T> {
    coeff: T,
    a_prime: Vec<T>,
    c_prime: Vec<T>,
    m_inv: Vec<T>,
}

pub struct TridiagonalOperator<T, M>
where
    T: Real,
    M: Mesher1d<T>,
{
    pub lower: Vec<T>,
    pub diag: Vec<T>,
    pub upper: Vec<T>,
    pub _marker: std::marker::PhantomData<M>,
    cache: RefCell<Option<TridiagonalCache<T>>>,
}

impl<T, M> TridiagonalOperator<T, M>
where
    T: Real,
    M: Mesher1d<T>,
{
    pub fn new(lower: Vec<T>, diag: Vec<T>, upper: Vec<T>) -> Self {
        let n = diag.len();
        if n < 3 {
            panic!(
                "TridiagonalOperator requires at least 3 elements, found {}",
                n
            );
        }

        TridiagonalOperator {
            lower,
            diag,
            upper,
            _marker: PhantomData,
            cache: RefCell::new(None),
        }
    }
}

impl<T, M> LinearOperator<T, M> for TridiagonalOperator<T, M>
where
    T: Real,
    M: Mesher1d<T>,
{
    fn size(&self) -> usize {
        self.diag.len()
    }

    fn apply_into(&self, v: &[T], _t: T, out: &mut [T]) {
        let n = self.size();
        if n == 0 {
            return;
        }

        if n == 1 {
            out[0] = self.diag[0] * v[0];
            return;
        }

        out[0] = self.diag[0] * v[0] + self.upper[0] * v[1];

        for i in 1..n - 1 {
            out[i] = self.lower[i] * v[i - 1] + self.diag[i] * v[i] + self.upper[i] * v[i + 1];
        }

        let last = n - 1;
        out[last] = self.lower[last] * v[last - 1] + self.diag[last] * v[last];
    }

    fn setup_coeff(&self, coeff: T) {
        let mut cache = self.cache.borrow_mut();

        // Skip if already computed for this coeff
        if let Some(ref c) = *cache {
            if (c.coeff - coeff).abs() < T::from_f64(1e-12) {
                return;
            }
        }

        let n = self.size();
        let mut a_prime = vec![T::zero(); n];
        let mut c_prime = vec![T::zero(); n];
        let mut m_inv = vec![T::zero(); n];

        // Solve (I - coeff * L)x = b
        // Diagonal: d_i' = 1 - coeff * diag[i]
        // Off-diagonals: a_i = -coeff * lower[i], c_i = -coeff * upper[i]

        a_prime[0] = T::zero();

        let d0 = T::one() - coeff * self.diag[0];
        m_inv[0] = T::one() / d0;
        c_prime[0] = (-coeff * self.upper[0]) * m_inv[0];

        for i in 1..n {
            a_prime[i] = -coeff * self.lower[i];
            let d = T::one() - coeff * self.diag[i];
            let c = if i < n - 1 {
                -coeff * self.upper[i]
            } else {
                T::zero()
            };

            let m = d - a_prime[i] * c_prime[i - 1];
            m_inv[i] = T::one() / m;
            c_prime[i] = c * m_inv[i];
        }

        *cache = Some(TridiagonalCache {
            coeff,
            a_prime,
            c_prime,
            m_inv,
        });
    }

    fn solve_inverse_into(&self, b: &[T], _coeff: T, _t: T, dest: &mut [T], z_buffer: &mut [T]) {
        let cache = self.cache.borrow();
        let c = cache
            .as_ref()
            .expect("TridiagonalOperator: setup_coeff must be called first");
        let n = self.size();

        z_buffer[0] = b[0] * c.m_inv[0];
        for i in 1..n {
            z_buffer[i] = (b[i] - c.a_prime[i] * z_buffer[i - 1]) * c.m_inv[i];
        }

        // Back substitution: Use the precomputed c_prime
        dest[n - 1] = z_buffer[n - 1];
        for i in (0..n - 1).rev() {
            dest[i] = z_buffer[i] - c.c_prime[i] * dest[i + 1];
        }
    }

    fn solve_psor_into<C>(
        &self,
        b: &[T],
        coeff: T,
        constraint: &C,
        mesher: &M,
        x: &mut [T],
        _z_buffer: &mut [T],
    ) where
        C: Constraint<T, M>,
        M: Mesher1d<T>,
    {
        let n = self.size();
        let omega = T::from_f64(1.5);
        let tolerance = T::from_f64(1e-2);
        let max_iter = 1000; // Increased iterations for safety

        // A_ii = 1 - coeff * diag[i]
        // A_i,i-1 = -coeff * lower[i]
        // A_i,i+1 = -coeff * upper[i]

        for _ in 0..max_iter {
            let mut max_diff = T::zero();

            for i in 0..n {
                let old_xi = x[i];

                let mut sum = b[i];
                if i > 0 {
                    sum -= (-coeff * self.lower[i]) * x[i - 1]; // Use updated value x[i-1]
                }
                if i < n - 1 {
                    sum -= (-coeff * self.upper[i]) * x[i + 1]; // Use old value x[i+1]
                }

                let diag_val = T::one() - coeff * self.diag[i];

                // GS step
                let x_gs = sum / diag_val;

                // Relaxation
                let x_relaxed = (T::one() - omega) * old_xi + omega * x_gs;

                // Projection (The "P" in PSOR)
                x[i] = x_relaxed.max(constraint.lower_bound(i, mesher));

                max_diff = max_diff.max((x[i] - old_xi).abs());
            }

            if max_diff < tolerance {
                //println!("{}", j);
                break;
            }
        }
    }

    fn solve_ikonen_toivanen_into<C>(
        &self,
        b: &[T],
        coeff: T,
        constraint: &C,
        mesher: &M,
        x: &mut [T],
        z_buffer: &mut [T],
    ) where
        C: Constraint<T, M>,
    {
        let n = self.size();
        let tolerance = T::from_f64(1e-8);
        let max_iter = 100;

        // Ensure the cache is ready for the implicit operator (I - coeff * L)
        self.setup_coeff(coeff);

        // lambda tracks the Lagrange multipliers for the constraint
        let mut lambda = vec![T::zero(); n];
        let mut rhs_hat = vec![T::zero(); n];

        for _ in 0..max_iter {
            let mut max_diff = T::zero();

            // 1. Prepare modified RHS: b_hat = b + lambda
            for i in 0..n {
                rhs_hat[i] = b[i] + lambda[i];
            }

            // 2. Solve the tridiagonal system: (I - coeff * L) x_new = rhs_hat
            // We use the existing solve_inverse_into logic
            self.solve_inverse_into(&rhs_hat, coeff, T::zero(), x, z_buffer);

            // 3. Update lambda and enforce constraint
            for i in 0..n {
                let old_xi = x[i];
                let g_i = constraint.lower_bound(i, mesher);

                // Ikonen-Toivanen update:
                // x_new = max(g_i, x_calc - lambda_old)
                // lambda_new = max(0, lambda_old + g_i - x_calc)

                let val_minus_lambda = x[i] - lambda[i];
                x[i] = val_minus_lambda.max(g_i);

                let new_lambda = (lambda[i] + g_i - old_xi).max(T::zero());

                let diff = (new_lambda - lambda[i]).abs();
                if diff > max_diff {
                    max_diff = diff;
                }

                lambda[i] = new_lambda;
            }

            if max_diff < tolerance {
                break;
            }
        }
    }
}
