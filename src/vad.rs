use bunsen::{
    burner::tensor::TensorDataIndexView,
    kits::speech::silero_vad::{SileroVad, SileroVad16x8},
};
use burn::{Tensor, prelude::Backend, tensor::backend::BackendTypes};

use crate::{SeppleBackend, ipa_recognizer::samples_to_tensor};

/// For 16khz audio silero works on chunks of 512
pub const CHUNK_SIZE: usize = 512;

pub struct Vad<B: Backend + BackendTypes = SeppleBackend> {
    device: B::Device,
    vad: SileroVad<B>,
    hidden_state: Tensor<B, 3>,
    context: Tensor<B, 2>,
}

impl<B: Backend> Vad<B> {
    pub fn init() -> Self {
        let device = B::Device::default();

        let vad = SileroVad16x8::load_from_burnpack("./model/silero_vad_op18_ifless.bpk", &device)
            .expect("Failed to load silero vad")
            .vad16;

        let hidden_state = vad.init_state(1, &device);
        let context = vad.init_context(1, &device);

        Self {
            device,
            vad,
            hidden_state,
            context,
        }
    }

    pub fn process_audio(&mut self, audio: &[f32]) -> Vec<f32> {
        audio
            .chunks_exact(CHUNK_SIZE)
            .by_ref()
            .map(|chunk| self.process_chunk(chunk))
            .collect()
    }

    pub fn process_chunk(&mut self, audio: &[f32]) -> f32 {
        TensorDataIndexView::<f32>::view(&self.forward_chunk(audio).into_data().convert::<f32>())
            [&[0, 0]]
    }

    fn forward_chunk(&mut self, samples: &[f32]) -> Tensor<B, 2> {
        let tensor = samples_to_tensor(samples, &self.device);
        let (probabilities, next_state, next_context) =
            self.vad
                .forward(tensor, self.hidden_state.clone(), self.context.clone());
        self.hidden_state = next_state;
        self.context = next_context;
        probabilities
    }
}
