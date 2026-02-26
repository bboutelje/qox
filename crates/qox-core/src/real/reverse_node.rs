use std::cell::RefCell;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::traits::real::Real; // Adjust this path if necessary

// --- 1. Hidden Tape Mechanism ---

thread_local! {
    static TAPE: RefCell<Vec<Node>> = RefCell::new(Vec::new());
}

struct Node {
    weights: Vec<(usize, f64)>, 
}

fn push_to_tape(weights: Vec<(usize, f64)>) -> usize {
    TAPE.with(|t| {
        let mut nodes = t.borrow_mut();
        let idx = nodes.len();
        nodes.push(Node { weights });
        idx
    })
}

// --- 2. The Struct Definition ---

#[derive(Clone, Copy, Debug)]
pub struct ReverseGradient {
    pub val: f64,
    pub index: usize,
}

impl ReverseGradient {
    pub fn var(val: f64) -> Self {
        Self { val, index: push_to_tape(vec![]) }
    }

    pub fn reset_tape() {
        TAPE.with(|t| t.borrow_mut().clear());
    }

    pub fn backward(&self) -> Vec<f64> {
        TAPE.with(|t| {
            let nodes = t.borrow();
            let mut gradients = vec![0.0; nodes.len()];
            gradients[self.index] = 1.0;

            for i in (0..nodes.len()).rev() {
                let grad = gradients[i];
                if grad == 0.0 { continue; }
                for &(parent_idx, weight) in &nodes[i].weights {
                    gradients[parent_idx] += weight * grad;
                }
            }
            gradients
        })
    }
}

// --- 3. Satisfying Supertrait Bounds (Foundation) ---

impl From<f64> for ReverseGradient {
    fn from(v: f64) -> Self {
        Self { val: v, index: push_to_tape(vec![]) }
    }
}

impl PartialEq for ReverseGradient {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

// PartialOrd is often required if your Real trait uses it for max()
impl PartialOrd for ReverseGradient {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

// --- 4. Operator Overloads (Owned + Reference) ---

impl<'a> Add<&'a ReverseGradient> for ReverseGradient {
    type Output = Self;
    fn add(self, rhs: &'a Self) -> Self {
        Self { val: self.val + rhs.val, index: push_to_tape(vec![(self.index, 1.0), (rhs.index, 1.0)]) }
    }
}

impl<'a, 'b> Add<&'b ReverseGradient> for &'a ReverseGradient {
    type Output = ReverseGradient;
    fn add(self, rhs: &'b ReverseGradient) -> Self::Output { (*self).add(rhs) }
}

impl<'a> Sub<&'a ReverseGradient> for ReverseGradient {
    type Output = Self;
    fn sub(self, rhs: &'a Self) -> Self {
        Self { val: self.val - rhs.val, index: push_to_tape(vec![(self.index, 1.0), (rhs.index, -1.0)]) }
    }
}

impl<'a, 'b> Sub<&'b ReverseGradient> for &'a ReverseGradient {
    type Output = ReverseGradient;
    fn sub(self, rhs: &'b ReverseGradient) -> Self::Output { (*self).sub(rhs) }
}

impl<'a> Mul<&'a ReverseGradient> for ReverseGradient {
    type Output = Self;
    fn mul(self, rhs: &'a Self) -> Self {
        Self { val: self.val * rhs.val, index: push_to_tape(vec![(self.index, rhs.val), (rhs.index, self.val)]) }
    }
}

impl<'a, 'b> Mul<&'b ReverseGradient> for &'a ReverseGradient {
    type Output = ReverseGradient;
    fn mul(self, rhs: &'b ReverseGradient) -> Self::Output { (*self).mul(rhs) }
}

impl<'a> Div<&'a ReverseGradient> for ReverseGradient {
    type Output = Self;
    fn div(self, rhs: &'a Self) -> Self {
        let v2 = rhs.val * rhs.val;
        Self { val: self.val / rhs.val, index: push_to_tape(vec![(self.index, 1.0 / rhs.val), (rhs.index, -self.val / v2)]) }
    }
}

impl<'a, 'b> Div<&'b ReverseGradient> for &'a ReverseGradient {
    type Output = ReverseGradient;
    fn div(self, rhs: &'b ReverseGradient) -> Self::Output { (*self).div(rhs) }
}

impl Neg for ReverseGradient {
    type Output = Self;
    fn neg(self) -> Self {
        Self { val: -self.val, index: push_to_tape(vec![(self.index, -1.0)]) }
    }
}

impl<'a> Neg for &'a ReverseGradient {
    type Output = ReverseGradient;
    fn neg(self) -> Self::Output { -(*self) }
}

// --- 5. Final Real Trait Implementation ---

impl Real for ReverseGradient {
    fn from_f64(v: f64) -> Self {
        Self::from(v)
    }

    fn to_f64(&self) -> f64 {
        self.val
    }

    fn max(&self, other: &Self) -> Self {
        let (val, w1, w2) = if self.val >= other.val { (self.val, 1.0, 0.0) } else { (other.val, 0.0, 1.0) };
        Self { val, index: push_to_tape(vec![(self.index, w1), (other.index, w2)]) }
    }

    fn exp(&self) -> Self {
        let res = self.val.exp();
        Self { val: res, index: push_to_tape(vec![(self.index, res)]) }
    }

    fn ln(&self) -> Self {
        Self { val: self.val.ln(), index: push_to_tape(vec![(self.index, 1.0 / self.val)]) }
    }

    fn sqrt(&self) -> Self {
        let res = self.val.sqrt();
        Self { val: res, index: push_to_tape(vec![(self.index, 1.0 / (2.0 * res))]) }
    }

    fn powi(&self, n: i32) -> Self {
        let factor = (n as f64) * self.val.powi(n - 1);
        Self { val: self.val.powi(n), index: push_to_tape(vec![(self.index, factor)]) }
    }

    fn powf(&self, n: &Self) -> Self {
        let val = self.val.powf(n.val);
        let d_base = n.val * self.val.powf(n.val - 1.0);
        let d_exp = val * self.val.ln();
        Self { val, index: push_to_tape(vec![(self.index, d_base), (n.index, d_exp)]) }
    }

    fn norm_cdf(&self) -> Self {
        let val = 0.5 * (1.0 + erf(self.val / 2.0f64.sqrt()));
        let pdf = (-0.5 * self.val * self.val).exp() / (2.0 * std::f64::consts::PI).sqrt();
        Self { val, index: push_to_tape(vec![(self.index, pdf)]) }
    }
}

// Internal helper for norm_cdf
fn erf(x: f64) -> f64 {
    let p = 0.3275911;
    let a = [0.254829592, -0.284496736, 1.421413741, -1.453152027, 1.061405429];
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - ((((a[4]*t + a[3])*t + a[2])*t + a[1])*t + a[0])*t * (-x*x).exp();
    sign * y
}