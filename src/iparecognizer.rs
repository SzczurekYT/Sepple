use std::{collections::HashMap, fs};

use burn::{Tensor, prelude::Backend, tensor::backend::BackendTypes};

use multipa_model::Model;
use serde_json::Value;

pub struct IpaRecognizer<B: Backend + BackendTypes> {
    device: B::Device,
    model: Model<B>,
    vocab: HashMap<i32, String>,
    padding_token_id: i32,
}

impl<B: Backend + BackendTypes> IpaRecognizer<B> {
    pub fn init() -> Self {
        let vocab = load_vocab("model/vocab.json");
        let padding_token_id = load_padding_token_id_from_config("model/config.json");

        Self {
            device: B::Device::default(),
            model: Model::default(),
            vocab,
            padding_token_id,
        }
    }

    /// Takes normalized audio samples, returns vector of non decoded token ids
    pub fn process(&self, input: &[f32]) -> Vec<i32> {
        let input_tensor = samples_to_tensor(input, &self.device);
        let logits = self.model.forward(input_tensor);
        let logits_2d = logits.squeeze::<2>();
        let predicted_ids = logits_2d.argmax(1);

        predicted_ids
            .to_data()
            .into_vec::<i32>()
            .expect("Failed to convert tensor to Vec<i32>")
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

    pub fn decode_tokens(&self, tokens: &[i32]) -> String {
        tokens
            .iter()
            .map(|id| {
                self.vocab
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| "?".to_string())
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

pub fn load_vocab(path: &str) -> HashMap<i32, String> {
    let data = fs::read_to_string(path).expect("Unable to read vocab.json");
    let map: HashMap<String, i32> = serde_json::from_str(&data).expect("Invalid vocab.json format");

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

pub fn samples_to_tensor<B>(samples: &[f32], device: &B::Device) -> Tensor<B, 2>
where
    B: Backend + BackendTypes,
{
    // reshape to [1, len]
    Tensor::<B, 1>::from_floats(samples, device).unsqueeze_dim(0)
}

pub fn z_score_normalize(input: &[f32]) -> Vec<f32> {
    const EPSILON: f32 = 1e-9;
    let len = input.len() as f32;
    let mean = input.iter().sum::<f32>() / len;
    let variance: f32 = input
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f32>()
        / len;
    let std_deviation = variance.sqrt();
    input
        .iter()
        .map(|value| (value - mean) / (std_deviation + EPSILON))
        .collect()
}
