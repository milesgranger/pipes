use crate::Transformer;
use std::marker::PhantomData;

pub struct MinMaxScaler<I, O> {
    min: Option<I>,
    max: Option<I>,
    phantom: PhantomData<O>,
}

impl<I, O> MinMaxScaler<I, O> {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            phantom: PhantomData,
        }
    }
    pub fn max(&self) -> Option<&I> {
        self.max.as_ref()
    }
    pub fn min(&self) -> Option<&I> {
        self.min.as_ref()
    }
}

impl<I, O> Transformer<I, O> for MinMaxScaler<I, O>
where
    I: Ord + Copy,
{
    type Input = I;
    type Output = I;

    fn fit(&mut self, x: Vec<Self::Input>) {
        self.max = Some(*x.iter().max().unwrap());
        self.min = Some(*x.iter().min().unwrap());
    }

    fn transform(&self, x: Vec<Self::Input>) -> Vec<Self::Input> {
        x.into_iter()
            .map(|v| {
                if Some(v) > self.max {
                    self.max.unwrap()
                } else if Some(v) < self.min {
                    self.min.unwrap()
                } else {
                    v
                }
            })
            .collect()
    }
}
