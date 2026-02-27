use crate::traits::real::{Real};

pub trait Mesher1d<T: Real>
{
    fn size(&self) -> usize;
    fn centers(&self) -> &[T];
    fn h_plus(&self) -> &[T];
    fn h_minus(&self) -> &[T];
    
    fn location(&self, index: usize) -> &T {
        &self.centers()[index]   
    }
}