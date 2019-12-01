#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]

pub mod minmax;
pub use minmax::*;

pub struct Pipeline<I, O> {
    steps: Vec<Box<dyn Transformer<I, O, Input = I, Output = O>>>,
}

impl<I, O> Pipeline<I, O> {
    pub fn new(steps: Vec<Box<dyn Transformer<I, O, Input = I, Output = O>>>) -> Self {
        Self { steps }
    }
}

pub trait Transformer<I, O> {
    type Input = I;
    type Output = O;
    fn transform(&self, x: Vec<Self::Input>) -> Vec<Self::Output>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
