import sys
import torch
from silero_vad import (
    load_silero_vad,
    read_audio,
)


# This is a testing file for comparing the Rust
# Silero impl with results rom python.

path = sys.argv[1]

torch.set_num_threads(1)

SAMPLING_RATE = 16000

model = load_silero_vad(onnx=False, opset_version=16)

wav = read_audio(path, sampling_rate=SAMPLING_RATE)

speech_probs = []
window_size_samples = 512 if SAMPLING_RATE == 16000 else 256

for i in range(0, len(wav), window_size_samples):
    chunk = wav[i : i + window_size_samples]
    if len(chunk) < window_size_samples:
        break
    speech_prob = model(chunk, SAMPLING_RATE).item()
    speech_probs.append(f"{speech_prob:.6f}")
model.reset_states()  # reset model states after each audio

print(str(speech_probs).replace("'", ""))
