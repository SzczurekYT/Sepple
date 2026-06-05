import torch
import subprocess
import os
from transformers import Wav2Vec2ForCTC

MODEL_PATH = "model/multipa.onnx"
SIMPLIFIED_MODEL_PATH = "model/multipa_sim.onnx"

dummy_input = torch.randn(1, 16000)

model = Wav2Vec2ForCTC.from_pretrained(
    "ctaguchi/wav2vec2-large-xlsr-japlmthufielta-ipa1000-ns"
)
model.eval()

onnx_program = torch.onnx.export(
    model,
    dummy_input,
    dynamo=True,
    input_names=["input_values"],
    output_names=["logits"],
    dynamic_axes={
        "input_values": {0: "batch", 1: "audio_length"},  # batch and length are dynamic
        "logits": {0: "batch", 1: "time_steps"},  # output time steps will vary
    },
)
onnx_program.save(MODEL_PATH)

# To make the model work with Burn right now it needs to be simplified with `onnxsim`

print("Simplyfying with onnxsim (this may take a while)")

result = subprocess.run(
    [".venv/bin/onnxsim", MODEL_PATH, SIMPLIFIED_MODEL_PATH],
    capture_output=True,
    text=True,
    # check=True
)
print(result.stdout)
print(result.stderr)

if result.returncode != 0:
    print("Running onnxsim failed")
    exit()

os.remove(MODEL_PATH)

print("Export to multipa_sim.onnx done")
