// use crate::{
//     methods::{
//         constraints::Constraint,
//         finite_difference::{free_boundary::FreeBoundaryStrategy, meshers::Mesher1d},
//         linear_operators_old::LinearOperator,
//     },
//     types::Real,
// };

// pub struct PsorConstrained<C> {
//     pub constraint: C,
// }
// impl<T: Real, M: Mesher1d<T>, L: LinearOperator<T, M>, C: Constraint<T, M>>
//     FreeBoundaryStrategy<T, M, L> for PsorConstrained<C>
// {
//     fn solve_stage(&self, op: &L, b: &[T], coeff: T, _t: T, m: &M, dest: &mut [T], z: &mut [T]) {
//         op.solve_psor_into(b, coeff, &self.constraint, m, dest, z);
//     }

//     fn compute_stage_derivative<IC>(
//         &self,
//         operator: &L,
//         stage_slice: &[T],
//         next_t: T,
//         mesher: &M,
//         initial_conditions: IC,
//         l_stage_slice: &mut [T],
//     ) where
//         IC: crate::traits::payoff::InitialConditions<T> + Copy,
//     {
//         operator.apply_into(stage_slice, next_t, l_stage_slice);

//         for j in 0..operator.size() {
//             let s = mesher.location(j);
//             let payoff = initial_conditions.get_value(s);

//             // If we are at or below the payoff (for a Put) or above (for a Call),
//             // the value is 'pinned', so the time derivative f(y) effectively becomes 0.
//             if stage_slice[j] <= payoff + T::from_f64(f64::EPSILON) {
//                 l_stage_slice[j] = T::zero();
//             }
//         }
//     }
// }
