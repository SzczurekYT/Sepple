use crate::pipeline::{PipelineConsumer, PipelineSink};

pub struct AssertStringPrinter {
    reference: Vec<String>,
    values: Vec<String>,
}

impl AssertStringPrinter {
    pub fn new(reference: Vec<String>) -> Self {
        Self {
            values: vec![],
            reference,
        }
    }
}

impl PipelineConsumer for AssertStringPrinter {
    type Input = String;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineSink for AssertStringPrinter {
    fn name() -> &'static str {
        "ValuePrinter"
    }

    async fn consume_value(&mut self, value: String) -> () {
        self.values.push(value.clone());
        println!("Got: {value}");
    }
}

impl Drop for AssertStringPrinter {
    fn drop(&mut self) {
        for (index, reference) in self.reference.iter().enumerate() {
            let value = &self.values[index];
            assert_eq!(value, reference, "\nIndex: {index}");
        }
    }
}
