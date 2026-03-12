use crate::{traits::{real::Real, transform::Transform}};

pub struct BlackScholesProcess<T: Real, Tr: Transform<T>> {
    pub r: T,
    pub sigma: T,
    pub transform: Tr,
    _marker: std::marker::PhantomData<T>,
}

pub struct BlackScholesOperator<T> {
    pub lower: Vec<T>,
    pub diag: Vec<T>,
    pub upper: Vec<T>,
}

impl<T: Real, Tr: Transform<T>> BlackScholesProcess<T, Tr> {
    pub fn build_operator(&self, grid: &[T]) -> BlackScholesOperator<T> {
        let n = grid.len();
        let mut lower = vec![T::zero(); n];
        let mut diag = vec![T::zero(); n];
        let mut upper = vec![T::zero(); n];

        if n < 2 { return BlackScholesOperator { lower, diag, upper }; }

        // Step 1: Calculate Delta X (assuming a uniform grid in transformed space)
        let dx = grid[1] - grid[0];
        let dx2 = dx * dx;
        let two = T::from_f64(2.0);

        // Step 2: Interior Points (Finite Difference Discretization)
        for i in 1..n - 1 {
            let xi = grid[i];
            let j = self.transform.jacobian(xi);
            let h = self.transform.hessian(xi);
            
            // Raw PDE coefficients: a(d2/dx2) + b(d/dx) + c(I)
            let (a, b, c) = self.stencil(xi, j, h);
            
            // Map coefficients to the tridiagonal matrix using Central Differences
            lower[i] = (a / dx2) - (b / (two * dx));
            diag[i]  = (T::from_f64(-2.0) * a / dx2) + c;
            upper[i] = (a / dx2) + (b / (two * dx));
        }

        diag[0] = T::one();
        upper[0] = T::from_f64(-1.0);

        lower[n-1] = T::from_f64(-1.0);
        diag[n-1] = T::one();

        BlackScholesOperator { lower, diag, upper }
    }

    fn stencil(&self, xi: T, j: T, h: T) -> (T, T, T) {
        let s = self.transform.to_physical(xi);
        let s2_sig2 = (s * self.sigma) * (s * self.sigma);
        let two = T::from_f64(2.0);

        let a = s2_sig2 / (two * j * j);
        let b = (self.r * s / j) - (s2_sig2 * h) / (two * j * j * j);
        let c = -self.r;

        (a, b, c)
    }

    fn stencil_d_sigma(&self, xi: T, j: T, h: T) -> (T, T, T) {
        let s = self.transform.to_physical(xi);
        let two = T::from_f64(2.0);
        
        // a = (s^2 * sigma^2) / (2 * j^2)
        // da/dsigma = (s^2 * 2 * sigma) / (2 * j^2) = (s^2 * sigma) / j^2
        let da = (s * s * self.sigma) / (j * j);
        
        // b = (r*s / j) - (s^2 * sigma^2 * h) / (2 * j^3)
        // db/dsigma = - (s^2 * 2 * sigma * h) / (2 * j^3) = - (s^2 * sigma * h) / j^3
        let db = - (s * s * self.sigma * h) / (j * j * j);
        
        // c = -r -> dc/dsigma = 0
        (da, db, T::zero())
    }

    // Returns (da/dr, db/dr, dc/dr)
    fn stencil_d_r(&self, xi: T, j: T, h: T) -> (T, T, T) {
        let s = self.transform.to_physical(xi);
        // a is independent of r
        // b = (r*s / j) - ... -> db/dr = s / j
        // c = -r -> dc/dr = -1
        (T::zero(), self.transform.to_physical(xi) / self.transform.jacobian(xi), T::from_f64(-1.0))
    }
}