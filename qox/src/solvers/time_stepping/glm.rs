use crate::traits::{linear_operator::LinearOperator, real::Real};

pub struct GlmTableau<T, const S: usize, const R: usize> {
    pub a: [[T; S]; S],
    pub u: [[T; R]; S],
    pub b: [[T; S]; R],
    pub v: [[T; R]; R],
    pub c: [T; S],
}

pub struct GlmState<T> {
    pub items: Vec<T>, // Length is R * N
    pub r: usize,      // Number of history steps (e.g., 1 for Euler/SDIRK, 2 for BDF2)
    pub n: usize,      // Number of grid nodes
    pub current_time: T,
}

impl<T: Real> GlmState<T> {
    pub fn new(r: usize, n: usize, current_time: T) -> Self {
        Self {
            items: vec![T::zero(); r * n],
            r,
            n,
            current_time,
        }
    }

    #[inline(always)]
    pub fn step_slice(&self, j: usize) -> &[T] {
        let start = j * self.n;
        &self.items[start..start + self.n]
    }

    #[inline(always)]
    pub fn step_slice_mut(&mut self, j: usize) -> &mut [T] {
        let start = j * self.n;
        &mut self.items[start..start + self.n]
    }
}

pub struct GlmWorkspace<T> {
    pub stages: Vec<T>,   // Flattened [S * N]
    pub l_stages: Vec<T>, // Flattened [S * N]
    pub rhs_buffer: Vec<T>, // [N]
    pub z_buffer: Vec<T>,
}

impl<T: Real> GlmWorkspace<T> {
    pub fn new(s: usize, n: usize) -> Self {
        let zero = T::zero();
        Self {
            stages: vec![zero; s * n],
            l_stages: vec![zero; s * n],
            rhs_buffer: vec![zero; n],
            z_buffer: vec![zero; n],
        }
    }
}

// impl<T: Real> GlmState<T> {
//     pub fn new(r: usize, n: usize, current_time: T) -> Self {
//         Self {
//             items: vec![T::zero(); r * n],
//             r,
//             n,
//             current_time,
//         }
//     }
// }

// pub fn step_glm_fast<T, Op, const S: usize, const R: usize>(
//     op: &Op,
//     tableau: &GlmTableau<T, S, R>,
//     state: &mut GlmState<T>,
//     ws: &mut GlmWorkspace<T>,
//     dt: T,
// ) where T: Real, Op: LinearOperator<T> 
// {
//     let n = state.n;
    
//     for i in 0..S {
//         let stage_time = state.current_time + tableau.c[i] * dt;
        
//         ws.rhs_buffer.fill(T::zero());

//         // 1. History contribution (U matrix)
//         // This is now a cache-friendly contiguous read
//         for j in 0..R {
//             let weight = tableau.u[i][j];
//             let prev_step = state.step_slice(j);
//             for k in 0..n {
//                 ws.rhs_buffer[k] += weight * prev_step[k];
//             }
//         }

//         // 2. Previous stages contribution (A matrix)
//         for j in 0..i {
//             let weight = dt * tableau.a[i][j];
//             let prev_l_stage = &ws.l_stages[j * n .. (j + 1) * n];
//             for k in 0..n {
//                 ws.rhs_buffer[k] += weight * prev_l_stage[k];
//             }
//         }

//         // 3. Solve (Implicit or Explicit)
//         let diag_coeff = dt * tableau.a[i][i];
//         let stage_offset = i * n;
//         let current_stage = &mut ws.stages[stage_offset..stage_offset + n];
        
//         if diag_coeff == T::zero() {
//             current_stage.copy_from_slice(&ws.rhs_buffer);
//         } else {
//             // solve_inverse should write directly into the workspace slice
//             op.solve_inverse_into(&ws.rhs_buffer, diag_coeff, stage_time, current_stage);
//         }

//         // 4. Apply L and cache in l_stages
//         let current_l_stage = &mut ws.l_stages[stage_offset..stage_offset + n];
//         op.apply_into(current_stage, stage_time, current_l_stage);
//     }

//     // ... Final Assembly into state.data ...
// }

