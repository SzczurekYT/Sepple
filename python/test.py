import sys
import torch
from torchcodec.decoders import WavDecoder
from transformers import Wav2Vec2ForCTC, Wav2Vec2Processor

# This is a testing file that does many of the processing steps manually
# giving us access to raw values betweem them, which
# is useful for comparsion with the Rust impl

path = sys.argv[1]

decoder = WavDecoder(path)

waveform = decoder.get_all_samples().data

MODEL_NAME = "ctaguchi/wav2vec2-large-xlsr-japlmthufielta-ipa1000-ns"
model = Wav2Vec2ForCTC.from_pretrained(MODEL_NAME)
processor = Wav2Vec2Processor.from_pretrained(MODEL_NAME)
model.eval()

waveform_np = waveform.squeeze().numpy()

mean = waveform_np.mean()
std = waveform_np.std()
epsilon = 1e-9
normalized = (waveform_np - mean) / (std + epsilon)

input_tensor = torch.tensor(normalized).unsqueeze(0)

with torch.no_grad():
    logits = model(input_tensor).logits

print("Python logits shape:", logits.shape)
print("First 5 logits (time step 0):", logits[0, 0, :5].tolist())

predicted_ids = torch.argmax(logits, dim=-1).squeeze(0)
blank_id = model.config.pad_token_id
print(f"Pad token id {blank_id}")

tokens = []
prev = None
for p in predicted_ids.tolist():
    if p == blank_id:
        prev = None
    elif p != prev:
        tokens.append(p)
        prev = p

print("Token IDs (first 30):", tokens[:30])
print("Decoded IPA:", processor.decode(tokens))
