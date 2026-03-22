// use crate::{
//     methods::{
//         constraints::Constraint,
//         finite_difference::meshers::{Mesher1d, SpatialGrid},
//         linear_operators_old::LinearOperator,
//     },
//     types::Real,
// };

// pub struct BsOperatorCache<T> {
//     pub coeff: T,
//     pub a: T,
//     pub b: T,
//     pub c: T,
//     pub c_prime: Vec<T>,
//     pub m_inv: Vec<T>,
// }

// pub struct BsOperator<'a, T: Real, M: Mesher1d<T>> {
//     pub mesher: &'a M,
//     pub r: T,
//     pub sigma: T,
//     pub cache: std::cell::RefCell<Option<BsOperatorCache<T>>>,
// }

// impl<'a, T, SG> LinearOperator<T, SG> for BsOperator<'a, T, SG>
// where
//     T: Real,
//     M: SpatialGrid<T>,
// {
//     fn size(&self) -> usize {
//         self.mesher.size()
//     }

//     fn apply_into(&self, v: &[T], _t: T, out: &mut [T]) {
//         let (l, d, u) = self.get_stencil();
//         let n = self.size();

//         out[0] = T::zero();

//         for i in 1..n - 1 {
//             out[i] = l * v[i - 1] + d * v[i] + u * v[i + 1];
//         }

//         out[n - 1] = T::zero();
//     }

//     fn solve_inverse_into(&self, b: &[T], _coeff: T, _t: T, dest: &mut [T], z_buffer: &mut [T]) {
//         let cache = self.cache.borrow();
//         let c = cache.as_ref().expect("BsOperatorCache not initialized");

//         let n = self.size();

//         // Copy RHS
//         dest.copy_from_slice(b);

//         // Forward sweep (Thomas)
//         z_buffer[0] = dest[0];

//         for i in 1..n - 1 {
//             z_buffer[i] = (dest[i] - c.a * z_buffer[i - 1]) * c.m_inv[i];
//         }

//         z_buffer[n - 1] = dest[n - 1];

//         // Back substitution
//         dest[n - 1] = z_buffer[n - 1];

//         for i in (1..n - 1).rev() {
//             dest[i] = z_buffer[i] - c.c_prime[i] * dest[i + 1];
//         }

//         dest[0] = z_buffer[0];
//     }

//     fn setup_coeff(&self, coeff: T) {
//         let mut cache = self.cache.borrow_mut();

//         if let Some(ref c) = *cache {
//             if (c.coeff - coeff).abs() < T::from_f64(1e-12) {
//                 return;
//             }
//         }

//         let (l, d, u) = self.get_stencil();

//         let n = self.size();

//         // Tridiagonal matrix (I - coeff * L)
//         let a = -coeff * l;
//         let b = T::one() - coeff * d;
//         let c_val = -coeff * u;

//         let mut c_prime = vec![T::zero(); n];
//         let mut m_inv = vec![T::zero(); n];

//         // Thomas factorization
//         m_inv[1] = T::one() / b;
//         c_prime[1] = c_val * m_inv[1];

//         for i in 2..n - 1 {
//             let m = b - a * c_prime[i - 1];
//             m_inv[i] = T::one() / m;
//             c_prime[i] = c_val * m_inv[i];
//         }

//         *cache = Some(BsOperatorCache {
//             coeff,
//             a,
//             b,
//             c: c_val,
//             c_prime,
//             m_inv,
//         });
//     }

//     fn solve_psor_into<C>(
//         &self,
//         _b: &[T],
//         _coeff: T,
//         _constraint: &C,
//         _mesher: &SG,
//         _x: &mut [T],
//         _z_buffer: &mut [T],
//     ) where
//         C: Constraint<T, M>,
//         M: Mesher1d<T>,
//     {
//         todo!()
//     }

//     fn solve_ikonen_toivanen_into<C>(
//         &self,
//         _b: &[T],
//         _coeff: T,
//         _constraint: &C,
//         _mesher: &M,
//         _x: &mut [T],
//         _z_buffer: &mut [T],
//     ) where
//         C: Constraint<T, M>,
//     {
//         todo!()
//     }
// }

// impl<'a, T, M> BsOperator<'a, T, M>
// where
//     T: Real,
//     M: Mesher1d<T>,
// {
//     fn get_stencil(&self) -> (T, T, T) {
//         let h = self.mesher.h_plus()[1];
//         let h2 = h * h;

//         let vol_sq = self.sigma * self.sigma;

//         let half = T::from_f64(0.5);
//         let two = T::from_f64(2.0);

//         let drift = self.r - half * vol_sq;
//         let diffusion = half * vol_sq;

//         let l = diffusion / h2 - drift / (two * h);
//         let d = diffusion * (-two) / h2 - self.r;
//         let u = diffusion / h2 + drift / (two * h);

//         (l, d, u)
//     }
// }
