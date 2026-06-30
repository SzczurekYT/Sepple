use std::{
    collections::VecDeque,
    io::{Write, stdout},
};

use bunsen::{
    burner::tensor::TensorDataIndexView,
    kits::speech::silero_vad::{SileroVad, pretrained::load_pretrained_silerovad},
};
use burn::{Tensor, prelude::Backend, tensor::backend::BackendTypes};

use crate::{SeppleBackend, iparecognizer::samples_to_tensor};

/// For 16khz audio silero works on chunks of 512
const SILERO_VAD_CHUNK_SIZE: usize = 512;

pub struct Vad<B: Backend + BackendTypes = SeppleBackend> {
    device: B::Device,
    vad: SileroVad<B>,
    hidden_state: Tensor<B, 3>,
    history: VecDeque<f32>,
}

impl<B: Backend> Vad<B> {
    pub fn init() -> Self {
        let device = B::Device::default();

        let vad = load_pretrained_silerovad::<B, _>("./model/silero_vad_op18_ifless.bpk", &device)
            .expect("Failed to load silero vad");

        let hidden_state = vad.init_state(1, &device);

        Self {
            device,
            vad,
            hidden_state,
            history: vec![0.0_f32; 31].into(),
        }
    }

    pub fn process_chunk(&mut self, audio: &[f32]) {
        assert_eq!(
            audio.len(),
            SILERO_VAD_CHUNK_SIZE,
            "Vad only accepts audio chunks of {SILERO_VAD_CHUNK_SIZE} samples"
        );
        let probability =
            TensorDataIndexView::<f32>::view(&self.forward_chunk(audio).into_data())[&[0, 0]];
        self.history.pop_front();
        self.history.push_back(probability);
        print!("\r{:.4?}", self.history);
        stdout().flush().unwrap();
    }

    fn forward_chunk(&mut self, samples: &[f32]) -> Tensor<B, 2> {
        let tensor = samples_to_tensor(samples, &self.device);
        let (probabilities, next_state) =
            self.vad.forward_sequence(tensor, self.hidden_state.clone());
        self.hidden_state = next_state;
        probabilities
    }
}
