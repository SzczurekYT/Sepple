import time
from transformers import pipeline
import sys

path = sys.argv[1]

# Load from the model directory
start_time = time.time()
# Model loaded from `model` directory
# Change to "ctaguchi/wav2vec2-large-xlsr-japlmthufielta-ipa1000-ns"
# if you want HF to take care of the download for you
pipe = pipeline("automatic-speech-recognition", model="model")
print(f"Loaded in {round(time.time() - start_time, 2)}")
start_time = time.time()
out = pipe(path)
print(f"Transcribed in {round(time.time() - start_time, 2)}")
print(out)
