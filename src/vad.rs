use bunsen::{
    burner::tensor::TensorDataIndexView,
    kits::speech::silero_vad::{SileroVad, SileroVadCollection, SileroVadContext},
};
use burn::{Tensor, prelude::Backend, tensor::backend::BackendTypes};

use crate::{SeppleBackend, ipa_recognizer::samples_to_tensor, units::SAMPLE_RATE_USIZE};

/// For 16khz audio silero works on chunks of 512
pub const CHUNK_SIZE: usize = 512;

pub struct Vad<B: Backend + BackendTypes = SeppleBackend> {
    device: B::Device,
    vad: SileroVad<B>,
    context: SileroVadContext<B>,
}

impl<B: Backend> Vad<B> {
    pub fn init() -> Self {
        let device = B::Device::default();

        let vad = SileroVadCollection::load_from_burnpack_file(
            "./model/silero_vad_op18_ifless.bpk",
            &device,
        )
        .expect("Failed to load silero vad")
        .branches
        .into_iter()
        .find(|(sample_rate, _)| *sample_rate == SAMPLE_RATE_USIZE)
        .expect("SileroVad does not support selected sample rate")
        .1;

        let context = vad.init_context(1, 64, &device);

        Self {
            device,
            vad,
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
            [&[0]]
    }

    fn forward_chunk(&mut self, samples: &[f32]) -> Tensor<B, 1> {
        let tensor = samples_to_tensor(samples, &self.device);
        let (probabilities, next_context) = self.vad.context_forward(tensor, self.context.clone());

        self.context = next_context;
        probabilities
    }
}
