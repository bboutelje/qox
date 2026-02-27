use crate::real::{dual_vec::DualVec64, dual2_vec::Dual2Vec64};

pub mod calendar;
pub mod instrument;
pub mod pricing_engine;
pub mod real;
pub mod rate_curve;
pub mod boundary;
pub mod fdm_1d_mesher;
pub mod vol_surface;



pub trait EvaluationResolver<RC, VS> {
    type Output;
}

// macro_rules! impl_eval_resolver {
//     ($D:ident) => {
//         // This handles the pure f64 case for the type
//         impl EvaluationResolver<f64, f64> for $D { type Output = $D; }
//         // Note: This version doesn't handle the <const N>
//     };

//     // Use this version for your Dual types with a const generic
//     (const $N:ident, $D:ident) => {
//         impl<const $N: usize> EvaluationResolver<f64, f64> for $D<$N> { type Output = $D<$N>; }
//         impl<const $N: usize> EvaluationResolver<$D<$N>, f64> for f64 { type Output = $D<$N>; }
//         impl<const $N: usize> EvaluationResolver<f64, $D<$N>> for f64 { type Output = $D<$N>; }
//         impl<const $N: usize> EvaluationResolver<$D<$N>, $D<$N>> for f64 { type Output = $D<$N>; }
//         impl<const $N: usize> EvaluationResolver<$D<$N>, f64> for $D<$N> { type Output = $D<$N>; }
//         impl<const $N: usize> EvaluationResolver<f64, $D<$N>> for $D<$N> { type Output = $D<$N>; }
//         impl<const $N: usize> EvaluationResolver<$D<$N>, $D<$N>> for $D<$N> { type Output = $D<$N>; }
//     };
// }

macro_rules! impl_eval_resolver {
    (const $N:ident, $D:ident) => {
        // --- 1. S is already the AD type ---
        // S(AD) + Rate(f64) + Vol(f64) -> AD
        impl<const $N: usize> EvaluationResolver<f64, f64> for $D<$N> { type Output = $D<$N>; }
        // S(AD) + Rate(AD) + Vol(f64) -> AD
        impl<const $N: usize> EvaluationResolver<$D<$N>, f64> for $D<$N> { type Output = $D<$N>; }
        // S(AD) + Rate(f64) + Vol(AD) -> AD
        impl<const $N: usize> EvaluationResolver<f64, $D<$N>> for $D<$N> { type Output = $D<$N>; }
        // S(AD) + Rate(AD) + Vol(AD) -> AD
        impl<const $N: usize> EvaluationResolver<$D<$N>, $D<$N>> for $D<$N> { type Output = $D<$N>; }

        // --- 2. S is f64 but promoted by other inputs ---
        // S(f64) + Rate(AD) + Vol(f64) -> AD
        impl<const $N: usize> EvaluationResolver<$D<$N>, f64> for f64 { type Output = $D<$N>; }
        // S(f64) + Rate(f64) + Vol(AD) -> AD
        impl<const $N: usize> EvaluationResolver<f64, $D<$N>> for f64 { type Output = $D<$N>; }
        // S(f64) + Rate(AD) + Vol(AD) -> AD
        impl<const $N: usize> EvaluationResolver<$D<$N>, $D<$N>> for f64 { type Output = $D<$N>; }
    };
}



// 1. Pure Path
impl EvaluationResolver<f64, f64> for f64 { type Output = f64; }

// 2. Dual Paths - Pass the "N" as a name for the macro to use in the impl header
impl_eval_resolver!(const N, DualVec64);
impl_eval_resolver!(const N, Dual2Vec64);


pub trait EvaluationResult<RC, VS>: EvaluationResolver<RC, VS> {
    type Out;
}

impl<S, RC, VS> EvaluationResult<RC, VS> for S 
where S: EvaluationResolver<RC, VS> {
    type Out = <S as EvaluationResolver<RC, VS>>::Output;
}
