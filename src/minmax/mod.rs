use crate::Transformer;
use std::marker::PhantomData;

pub struct MinMaxScaler<I, O> {
    min: I,
    max: I,
    phantom: PhantomData<O>,
}

impl<I, O> MinMaxScaler<I, O> {
    pub fn new(min: I, max: I) -> Self {
        Self {
            min,
            max,
            phantom: PhantomData,
        }
    }
}

impl<I, O> Transformer<I, O> for MinMaxScaler<I, O>
where
    I: PartialOrd + Copy,
{
    type Input = I;
    type Output = I;

    fn transform(&self, x: Vec<Self::Input>) -> Vec<Self::Input> {
        x.into_iter()
            .map(|v| {
                if v > self.max {
                    self.max
                } else if v < self.min {
                    self.min
                } else {
                    v
                }
            })
            .collect()
    }
}
