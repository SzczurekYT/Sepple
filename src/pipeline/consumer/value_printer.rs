use std::{fmt::Display, marker::PhantomData};

use crate::pipeline::{PipelineConsumer, PipelineSink};

pub struct ValuePrinter<T: Display + Send + 'static> {
    _phantom: PhantomData<T>,
}

impl<T: Display + Send + 'static> ValuePrinter<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: Display + Send + 'static> Default for ValuePrinter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display + Send + 'static> PipelineConsumer for ValuePrinter<T> {
    type Input = T;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl<T: Display + Send + 'static> PipelineSink for ValuePrinter<T> {
    fn name() -> &'static str {
        "ValuePrinter"
    }

    async fn consume_value(&mut self, value: Self::Input) -> () {
        println!("Got: {value}");
    }
}
