use std::{collections::HashMap, fs, path::Path};

use burn::{prelude::*, tensor::DType};
use burn_store::{BurnpackStore, ModuleSnapshot};
use serde_json::Value;
use zipa_model::ZipaSmallCrCtcNsModel;

use crate::{
    error::{SeppleError, SeppleResult},
    fbank::Fbank,
};

const PAD_TOKEN_ID: i32 = 0;
const VOCAB_JSON: &str = include_str!("../model/vocab.json");

pub struct IpaRecognizer {
    device: Device,
    model: ZipaSmallCrCtcNsModel,
    fbank: Fbank,
    vocab: HashMap<i32, String>,
    padding_token_id: i32,
}

impl IpaRecognizer {
    pub fn init_default<P>(model_path: P) -> SeppleResult<Self>
    where
        P: AsRef<Path>,
    {
        Self::init(model_path, PAD_TOKEN_ID, load_vocab_from_json(VOCAB_JSON))
    }

    pub fn init<P>(
        model_path: P,
        padding_token_id: i32,
        vocab: HashMap<i32, String>,
    ) -> SeppleResult<Self>
    where
        P: AsRef<Path>,
    {
        let device = Device::default();

        let mut model = ZipaSmallCrCtcNsModel::new(&device);
        let mut store = BurnpackStore::from_file(model_path);
        model
            .load_from(&mut store)
            .map_err(SeppleError::ModelLoad)?;

        let fbank = Fbank::new(device.clone());

        Ok(Self {
            model,
            device,
            fbank,
            vocab,
            padding_token_id,
        })
    }

    pub fn recognize(&self, input: &[f32]) -> String {
        let ids = self.process(input);
        let ids = self.greedy_ctc_decode(&ids);
        self.decode_tokens(&ids)
    }

    /// Takes raw 16 kHz mono audio samples, returns the decoded (pre-CTC) token ids.
    pub fn process(&self, input: &[f32]) -> Vec<i32> {
        let (log_probs, valid) = self.forward(input);

        log_probs
            .slice(s![0..valid, ..])
            .argmax(1)
            .into_data()
            .try_into_vec_as::<i32>()
            .expect("Failed to convert tensor to Vec<i32>")
    }

    /// Runs the model end to end. Returns the full `[frames, tokens]` log
    /// probabilities and the number of leading frames that are valid (the
    /// model's own output length).
    pub fn forward(&self, input: &[f32]) -> (Tensor<2>, usize) {
        let features = self.fbank.compute(input);
        let num_frames = features.dims()[0];

        let x = features.unsqueeze_dim::<3>(0);
        // The model's integer constants are int64, so `x_lens` must be too.
        let x_lens = Tensor::<1, Int>::from_data(
            TensorData::from([num_frames as i64]),
            (&self.device, DType::I64),
        );

        let (log_probs, log_probs_len) = self.model.forward(x, x_lens);
        let log_probs = log_probs.squeeze_dim::<2>(0);

        let valid = log_probs_len
            .into_data()
            .try_into_vec_as::<i64>()
            .expect("model output length to be integers")[0] as usize;

        (log_probs, valid)
    }

    pub fn greedy_ctc_decode(&self, ids: &[i32]) -> Vec<i32> {
        let mut tokens = Vec::new();
        let mut prev: Option<i32> = None;

        for &id in ids {
            if id == self.padding_token_id {
                prev = None;
            } else if prev != Some(id) {
                tokens.push(id);
                prev = Some(id);
            }
        }
        tokens
    }

    /// Returns a list of logits, and a list of indexes of each logit
    pub fn greedy_ctc_decode_with_indexes(&self, ids: &[i32]) -> (Vec<i32>, Vec<usize>) {
        let mut tokens = Vec::new();
        let mut indexes = Vec::new();
        let mut prev: Option<i32> = None;

        for (index, &id) in ids.iter().enumerate() {
            if id == self.padding_token_id {
                prev = None;
            } else if prev != Some(id) {
                tokens.push(id);
                indexes.push(index);
                prev = Some(id);
            }
        }
        (tokens, indexes)
    }

    pub fn decode_token(&self, token: i32) -> &str {
        self.vocab.get(&token).map(String::as_str).unwrap_or("?")
    }

    pub fn decode_tokens(&self, tokens: &[i32]) -> String {
        self.decode_each_token(tokens).concat()
    }

    pub fn decode_each_token(&self, tokens: &[i32]) -> Vec<&str> {
        tokens
            .iter()
            .copied()
            .map(|token| self.decode_token(token))
            .collect()
    }

    pub fn padding_token_id(&self) -> i32 {
        self.padding_token_id
    }
}

pub fn load_vocab_from_json(json: &str) -> HashMap<i32, String> {
    let map: HashMap<String, i32> = serde_json::from_str(json).expect("Invalid vocab.json format");

    map.into_iter().map(|(token, id)| (id, token)).collect()
}

pub fn load_padding_token_id_from_config(path: &str) -> i32 {
    let data = fs::read_to_string(path).expect("Unable to read config.json");
    let map: HashMap<String, Value> =
        serde_json::from_str(&data).expect("Invalid config.json format");

    map.get("pad_token_id")
        .expect("no pad_token_id in config.json")
        .as_i64()
        .expect("pad_token_id to be a number") as i32
}

pub fn samples_to_tensor(samples: &[f32], device: &Device) -> Tensor<2> {
    // reshape to [1, len]
    Tensor::<1>::from_floats(samples, device).unsqueeze_dim(0)
}

pub fn z_score_normalize<I>(input: I) -> Vec<f32>
where
    I: Iterator<Item = f32> + ExactSizeIterator + Clone,
{
    const EPSILON: f32 = 1e-9;
    let len = input.len() as f32;
    let mean = input.clone().sum::<f32>() / len;
    let variance: f32 = input
        .clone()
        .map(|value| (value - mean).powi(2))
        .sum::<f32>()
        / len;
    let std_deviation = variance.sqrt();
    input
        .map(|value| (value - mean) / (std_deviation + EPSILON))
        .collect()
}
