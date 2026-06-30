use std::{
    iter,
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread,
    time::Duration,
};

use bunsen::{
    burner::module::ModuleInit,
    kits::speech::silero_vad::{SileroVad, SileroVadConfig},
};
use burn::{Tensor, prelude::Backend, tensor::backend::BackendTypes};

use crate::{
    SeppleBackend,
    capture::AudioChunk,
    iparecognizer::{IpaRecognizer, samples_to_tensor},
    vad::Vad,
};

/// For 16khz audio silero works on chunks of 512
const SILERO_VAD_CHUNK_SIZE: usize = 512;

pub struct VadFilter {
    vad: Vad<SeppleBackend>,
}

impl VadFilter {
    pub fn init() -> Self {
        Self { vad: Vad::init() }
    }

    pub fn run(&mut self, receiver: Receiver<AudioChunk>, result_sender: Sender<AudioChunk>) {
        while let Ok(AudioChunk { timestamp, audio }) = receiver.recv() {
            self.vad.process_chunk(&audio);
            let send_result = result_sender.send(AudioChunk { timestamp, audio });
            if send_result.is_err() {
                break;
            }
        }
    }
}
