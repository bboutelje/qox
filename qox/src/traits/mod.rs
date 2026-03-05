use crate::real::{dual::Dual, dual_array::DualArray, num_dual_vec::NumDualVec};

pub mod calendar;
pub mod instrument;
pub mod pricing_engine;
pub mod real;
pub mod rate_curve;
pub mod boundary;
pub mod fdm_1d_mesher;
pub mod vol_surface;
pub mod payoff;
pub mod time_stepper;
pub mod linear_operator;

pub trait EvaluationResolver<RC, VS> {
    type Output;
}

pub type Resolved<SReal, RCReal, VSReal> = 
    <SReal as EvaluationResolver<RCReal, VSReal>>::Output;

macro_rules! impl_eval_resolver_simple {
    ($D:ident) => {
        // --- 1. S is already the AD type ---
        impl EvaluationResolver<f64, f64> for $D { type Output = $D; }
        impl EvaluationResolver<$D, f64> for $D { type Output = $D; }
        impl EvaluationResolver<f64, $D> for $D { type Output = $D; }
        impl EvaluationResolver<$D, $D> for $D { type Output = $D; }

        // --- 2. S is f64 but promoted ---
        impl EvaluationResolver<$D, f64> for f64 { type Output = $D; }
        impl EvaluationResolver<f64, $D> for f64 { type Output = $D; }
        impl EvaluationResolver<$D, $D> for f64 { type Output = $D; }
    };
}

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
impl_eval_resolver_simple!(Dual);
impl_eval_resolver!(const N, DualArray);
impl_eval_resolver!(const N, NumDualVec);



pub trait EvaluationResult<RC, VS>: EvaluationResolver<RC, VS> {
    type Out;
}

impl<S, RC, VS> EvaluationResult<RC, VS> for S 
where S: EvaluationResolver<RC, VS> {
    type Out = <S as EvaluationResolver<RC, VS>>::Output;
}
