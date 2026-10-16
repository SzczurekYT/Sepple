"""Dump the reference Zipa pipeline (lhotse fbank -> ONNX -> CTC decode).

Run with the zipa virtualenv, from the Sepple checkout:

    /home/szczurek/github/zipa/.venv/bin/python python/zipa_reference.py \\
        test.wav /tmp/opencode/ref
"""

import argparse
import os
import sys

import numpy as np

ZIPA = "/home/szczurek/github/zipa"
sys.path.insert(0, os.path.join(ZIPA, "inference"))

import onnxruntime as ort  # noqa: E402
import soundfile as sf  # noqa: E402
import torch  # noqa: E402
from utils import get_fbank_extractor, load_tokens  # noqa: E402

TOKENS = os.path.join(ZIPA, "ipa_simplified", "tokens.txt")
WORD_BOUNDARY = "\u2581"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("audio_file")
    parser.add_argument("out_dir")
    parser.add_argument(
        "--model-path",
        default=os.path.join(os.path.dirname(__file__), "..", "model", "zipa-small-crctc-ns.onnx"),
    )
    args = parser.parse_args()

    os.makedirs(args.out_dir, exist_ok=True)

    audio, sample_rate = sf.read(args.audio_file, dtype="float32")
    assert sample_rate == 16000 and audio.ndim == 1, "expected 16 kHz mono"

    extractor = get_fbank_extractor()
    audio_tensor = torch.from_numpy(audio).unsqueeze(0)
    feature = extractor.extract_batch([audio_tensor], sampling_rate=16000)[0].unsqueeze(0)
    fbank = feature[0].numpy()
    fbank.tofile(os.path.join(args.out_dir, "fbank.f32"))

    session = ort.InferenceSession(args.model_path)
    x_lens = np.array([feature.shape[1]], dtype=np.int64)
    log_probs, log_probs_len = session.run(None, {"x": feature.numpy(), "x_lens": x_lens})
    log_probs = log_probs[0]
    log_probs.tofile(os.path.join(args.out_dir, "log_probs.f32"))

    ids = log_probs[: log_probs_len[0]].argmax(axis=-1).astype(np.int32)
    ids.tofile(os.path.join(args.out_dir, "ids.i32"))

    vocab = load_tokens(TOKENS)
    prev = -1
    tokens = []
    for idx in ids.tolist():
        if idx != 0 and idx != prev:
            tokens.append(vocab.get(idx, ""))
        prev = idx
    text = "".join(token for token in tokens if token != WORD_BOUNDARY)
    with open(os.path.join(args.out_dir, "text.txt"), "w", encoding="utf-8") as out:
        out.write(text)

    print(
        f"fbank={fbank.shape} log_probs={log_probs.shape} "
        f"log_probs_len={log_probs_len[0]} ids={ids.shape}"
    )
    print(f"text={text}")


if __name__ == "__main__":
    main()
