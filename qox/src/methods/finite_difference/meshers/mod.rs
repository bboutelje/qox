use crate::types::Real;

pub mod concentrating;
pub mod log;
pub mod uniform;
pub mod uniform_old;

pub trait Mesher1d<T: Real> {
    fn size(&self) -> usize;
    fn centers(&self) -> &[T];
    fn h_plus(&self) -> &[T];
    fn h_minus(&self) -> &[T];

    fn location(&self, index: usize) -> T {
        self.centers()[index]
    }
}
