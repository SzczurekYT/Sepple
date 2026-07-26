use std::{env::args, sync::Arc};

use burn::backend::Flex;
use sepple::{ipa_recognizer::IpaRecognizer, read_wav_to_f32};
use tokio::{runtime::Handle, sync::Semaphore, task::JoinSet};

#[tokio::main]
async fn main() {
    let file = args().nth(1).expect("file");
    let total: usize = args().nth(1).expect("count").parse().expect("a number");
    let input = read_wav_to_f32(file);

    println!("Loading model");
    let recognizer = Arc::new(IpaRecognizer::<Flex>::init());
    println!("Load done");

    // Limit the number of work done at the same time to prevent consuming all RAM
    let semaphore = Arc::new(Semaphore::new(10));
    let mut set = JoinSet::new();

    for i in 0..total {
        let mut samples = vec![0.0; i];
        samples.extend(input.iter());

        let recognizer = Arc::clone(&recognizer);
        let semaphore = semaphore.clone();

        set.spawn_blocking(move || {
            let _permit = Handle::current().block_on(async { semaphore.acquire().await.unwrap() });

            println!("Start {i}");
            let result = recognizer.recognize(&samples);
            println!("Done {i}");

            (i, result)
        });
    }

    let mut values = set.join_all().await;

    values.sort_by_key(|(i, _)| *i);

    for (i, text) in values {
        println!("[{i:03}]: {text}");
    }
}
