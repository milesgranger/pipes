#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]

pub mod minmax;
pub use minmax::*;

pub struct Pipeline<I, O> {
    steps: Vec<Box<dyn Transformer<I, O, Input = I, Output = O>>>,
}

impl<I, O> Pipeline<I, O>
    where
        Vec<I>: From<Vec<O>> + Clone,
        Vec<O>: From<Vec<I>>,

{
    pub fn new(steps: Vec<Box<dyn Transformer<I, O, Input = I, Output = O>>>) -> Self {
        Self { steps }
    }

    pub fn steps(&self) -> &Vec<Box<dyn Transformer<I, O, Input = I, Output = O>>> {
        &self.steps
    }

    pub fn fit(&mut self, x: Vec<I>) {
        self.steps.iter_mut().fold(x, |acc, transformer| {
            transformer.fit(acc.clone());
            transformer.transform(acc.into()).into()
        });
    }
    pub fn transform(&self, x: Vec<I>) -> Vec<O> {
        let out = self.steps
            .iter()
            .fold(x, |out, transformer| {
                transformer.transform(out).into()
            });
        out.into()
    }
}

pub trait Transformer<I, O> {
    type Input = I;
    type Output = O;
    fn transform(&self, x: Vec<Self::Input>) -> Vec<Self::Output>;
    fn fit(&mut self, x: Vec<Self::Input>);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
