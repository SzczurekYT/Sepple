import time
from transformers import pipeline
import sys

path = sys.argv[1]

# Load from the model directory
start_time = time.time()
pipe = pipeline(
    "automatic-speech-recognition",
    model="ctaguchi/wav2vec2-large-xlsr-japlmthufielta-ipa1000-ns",
)
print(f"Loaded in {round(time.time() - start_time, 2)}")
start_time = time.time()
out = pipe(path, chunk_length_s=10, stride_length_s=(4, 2))
print(f"Transcribed in {round(time.time() - start_time, 2)}")
print(out)
