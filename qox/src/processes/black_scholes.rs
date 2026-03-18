use crate::{
    methods::{
        finite_difference::meshers::Mesher1d,
        linear_operators_old::tridiagonal_operator::TridiagonalOperator, transforms::Transform,
    },
    processes::FdmProcess,
    types::Real,
};

pub struct BlackScholesProcess<T: Real, Tr: Transform<T>> {
    pub r: T,
    pub sigma: T,
    pub transform: Tr,
}

impl<T, M, Tr> FdmProcess<T, TridiagonalOperator<T, M>, M, Tr> for BlackScholesProcess<T, Tr>
where
    T: Real,
    M: Mesher1d<T>,
    Tr: Transform<T> + Copy,
{
    fn build_operator(&self, mesher: &M) -> TridiagonalOperator<T, M> {
        let n = mesher.size();
        let centers = mesher.centers();
        let h_minus = mesher.h_minus();
        let h_plus = mesher.h_plus();

        let mut lower = vec![T::zero(); n];
        let mut diag = vec![T::zero(); n];
        let mut upper = vec![T::zero(); n];

        for i in 1..n - 1 {
            let xi = centers[i];
            let hm = h_minus[i];
            let hp = h_plus[i];

            let j = self.transform.jacobian(xi);
            let h = self.transform.hessian(xi);
            let (a, b, c) = self.stencil(xi, j, h);

            // Weights for non-uniform finite differences
            let denom = hm * hp * (hm + hp);

            // Second derivative term
            let a_lower = (T::from_f64(2.0) * hp) / denom;
            let a_diag = (T::from_f64(-2.0) * (hm + hp)) / denom;
            let a_upper = (T::from_f64(2.0) * hm) / denom;

            // First derivative term
            let b_lower = -(hp * hp) / denom;
            let b_diag = (hp * hp - hm * hm) / denom;
            let b_upper = (hm * hm) / denom;

            lower[i] = a * a_lower + b * b_lower;
            diag[i] = a * a_diag + b * b_diag + c;
            upper[i] = a * a_upper + b * b_upper;
        }

        // Boundary conditions
        diag[0] = T::one();
        upper[0] = T::zero();
        lower[n - 1] = T::zero();
        diag[n - 1] = T::one();

        TridiagonalOperator::<T, M>::new(lower, diag, upper)
    }

    fn transform(&self) -> Tr {
        self.transform
    }
}

impl<T: Real, Tr: Transform<T>> BlackScholesProcess<T, Tr> {
    pub fn new(rate: T, vol: T, transform: Tr) -> Self {
        Self {
            r: rate,
            sigma: vol,
            transform,
        }
    }

    fn stencil(&self, xi: T, j: T, h: T) -> (T, T, T) {
        let s = self.transform.to_physical(xi);
        let s_sig = s * self.sigma;
        let s2_sig2 = s_sig * s_sig;
        let two = T::from_f64(2.0);

        let j2 = j * j;
        let a = s2_sig2 / (two * j2);
        let b = (self.r * s / j) - (s2_sig2 * h) / (two * j2 * j);

        let c = -self.r;

        (a, b, c)
    }
}
