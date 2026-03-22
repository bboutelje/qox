// pub mod projection;
// pub mod psor;
// pub mod unconstrained;

// use crate::traits::payoff::InitialConditions;

// pub trait FreeBoundaryStrategy<T, M, L> {
//     fn solve_stage(
//         &self,
//         operator: &L,
//         rhs: &[T],
//         coeff: T,
//         next_t: T,
//         mesher: &M,
//         dest: &mut [T],
//         z_buffer: &mut [T],
//     );

//     fn compute_stage_derivative<IC>(
//         &self,
//         operator: &L,
//         stage_slice: &[T],
//         next_t: T,
//         mesher: &M,
//         initial_conditions: IC,
//         l_stage_slice: &mut [T],
//     ) where
//         IC: InitialConditions<T> + Copy;
// }
