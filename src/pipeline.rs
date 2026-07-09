use std::thread::{self};

use tokio::{
    runtime::{self},
    sync::mpsc::{self, Receiver, Sender, error::SendError},
};

const BOUNDED_CHANNEL_SIZE: usize = 10;

pub trait PipelineProducer
where
    Self: Sized + Send + 'static,
{
    type Output: Send;

    fn output_size(&self) -> Option<usize>;
}

pub trait PipelineConsumer
where
    Self: Sized + Send + 'static,
{
    type Input: Send;

    fn input_size(&self) -> Option<usize>;
}

pub trait PipelineSource: PipelineProducer {
    fn name() -> &'static str;

    fn run(&mut self, sender: Sender<Self::Output>) -> impl Future<Output = ()> + Send;

    fn to_future(
        mut self,
    ) -> (
        Receiver<Self::Output>,
        impl Future<Output = ()> + Send + 'static,
    )
    where
        Self: Sized + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel::<Self::Output>(BOUNDED_CHANNEL_SIZE);
        let future = async move {
            self.run(sender).await;
        };
        (receiver, future)
    }
}

pub trait PipelineProcessor: PipelineProducer + PipelineConsumer {
    fn name() -> &'static str;

    fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> impl Future<Output = Result<(), SendError<Self::Output>>> + Send;

    fn to_future(
        mut self,
        mut input_receiver: Receiver<Self::Input>,
    ) -> (Receiver<Self::Output>, impl Future<Output = ()> + Send)
    where
        Self: Sized + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel::<Self::Output>(BOUNDED_CHANNEL_SIZE);
        let future = async move {
            while let Some(value) = input_receiver.recv().await {
                let result = self.process_value(value, &sender).await;
                if result.is_err() {
                    break;
                }
            }
        };
        (receiver, future)
    }
}

pub trait PipelineSink: PipelineConsumer {
    fn name() -> &'static str;

    fn consume_value(&mut self, value: Self::Input) -> impl Future<Output = ()> + Send;

    fn to_future(
        mut self,
        mut input_receiver: Receiver<Self::Input>,
    ) -> impl Future<Output = ()> + Send {
        async move {
            while let Some(value) = input_receiver.recv().await {
                self.consume_value(value).await;
            }
        }
    }
}

pub struct PipelineThread {
    pub futures: Vec<Box<dyn Future<Output = ()> + Send>>,
}

impl PipelineThread {
    pub fn new() -> Self {
        Self { futures: vec![] }
    }
}

impl Default for PipelineThread {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Pipeline<T> {
    threads: Vec<PipelineThread>,
    output_size: Option<usize>,
    output_name: &'static str,
    receiver: Receiver<T>,
}

impl<T> Pipeline<T> {
    pub fn new<P>(producer: P) -> Pipeline<T>
    where
        P: PipelineSource<Output = T>,
    {
        let mut thread = PipelineThread::new();
        let output_size = producer.output_size();
        let (receiver, future) = producer.to_future();
        thread.futures.push(Box::new(future));
        let threads = vec![thread];
        Pipeline {
            threads,
            output_size,
            output_name: P::name(),
            receiver,
        }
    }

    pub fn then<P, O>(mut self, processor: P) -> Pipeline<O>
    where
        P: PipelineProcessor<Input = T, Output = O>,
    {
        check_size_contract(
            self.output_size,
            self.output_name,
            processor.input_size(),
            P::name(),
        );

        let output_size = processor.output_size();

        let (receiver, future) = processor.to_future(self.receiver);
        self.threads
            .last_mut()
            .expect("at least 1 thread")
            .futures
            .push(Box::new(future));

        Pipeline {
            threads: self.threads,
            output_name: P::name(),
            output_size,
            receiver,
        }
    }

    pub fn new_therad(mut self) -> Self {
        self.threads.push(PipelineThread::new());
        self
    }

    pub fn finish_and_run<C>(mut self, consumer: C)
    where
        C: PipelineSink<Input = T>,
    {
        check_size_contract(
            self.output_size,
            self.output_name,
            consumer.input_size(),
            C::name(),
        );

        let future = consumer.to_future(self.receiver);
        self.threads
            .last_mut()
            .expect("at least 1 thread")
            .futures
            .push(Box::new(future));

        let mut joins = Vec::with_capacity(self.threads.len());
        for thread in self.threads {
            joins.push(thread::spawn(|| {
                let rt = runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                let mut handles = Vec::with_capacity(thread.futures.len());

                for future in thread.futures {
                    let handle = rt.spawn(Box::into_pin(future));
                    handles.push(handle);
                }

                for handle in handles {
                    rt.block_on(handle).unwrap();
                }
            }));
        }

        joins.into_iter().for_each(|join| join.join().unwrap());
    }
}

fn check_size_contract(
    output_size: Option<usize>,
    output_name: &str,
    input_size: Option<usize>,
    input_name: &str,
) {
    let Some(required_input_size) = input_size else {
        return;
    };

    match output_size {
        Some(provided_input_size) if provided_input_size != required_input_size => {
            panic!(
                "{input_name} requires input of size {required_input_size} but {output_name} provides output of size {provided_input_size}",
            );
        }
        None => {
            panic!(
                "{input_name} requires input of size {required_input_size} but {output_name} has no output size guarantees",
            );
        }
        _ => {}
    }
}
