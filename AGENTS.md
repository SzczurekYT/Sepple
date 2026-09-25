# AGENTS.md

Sepple is a Rust library for transcribing speech to IPA, and detecting keyword in the IPA. It has a basic CLI, used mainly for testing the library.

## Running it

```shell
cargo run --release -- file -f <path.wav>              # IPA plus dictionary words
cargo run --release -- file -f <path.wav> --silero     # VAD only, writes speech_probabilities.wav
cargo run --release -- pipeline                        # live microphone
cargo run --release -- pipeline -f <path.wav>          # the full pipeline over a file
```

Add `-v` before the subcommand for more logging.

The first run downloads a 1.26 GB model from HuggingFace into the user cache
directory (`model_provider::get_model_path`). There is no flag to skip or
relocate it.

Input audio must be 16 kHz, mono, 16-bit PCM WAV. `read_wav_to_f32` asserts
all three and panics otherwise, and hound reads WAV only.

## Architecture

The workspace has three crates: `sepple` (library + CLI), `multipa-model`
(generated model code), and `trainer` (dataset building tools). Audio flows
from a source through a chain of pipeline processors into word detection.

### multipa-model

The multipa speech-to-IPA model itself. `multipa-model/src/multipa_sim.rs` is
generated from `model/multipa_sim.onnx` by burn-onnx and checked in; never
edit it by hand. `src/ipa_recognizer.rs` wraps it: it loads the 1.26 GB
burnpack weights, normalizes audio with a z-score, and greedy-CTC decodes
using the token vocabulary from `model/vocab.json`, which is compiled in with
`include_str!`.

### Pipeline processors

Processing runs as a chain of processors from `src/pipeline/`, joined by
bounded tokio channels (capacity 10, `src/pipeline.rs:20`). To change what
happens to the audio you add, remove, or reorder processors in `src/main.rs`
instead of editing one in place:

    MemoryAudioSource | AudioCapture
      -> AudioChunker
      -> SileroVadScorer
      -> VadFilter
      -> SlidingWindowChunker
      -> AudioLogger
      -> IpaProcessor
      -> WordDetector
      -> ValuePrinter

Sizes and thresholds are wired up in `run_pipeline` in `src/main.rs`, not
repeated here. The chain is built as
`Pipeline::new(source).then(processor)...build_and_run(sink)`. A processor is
any type implementing `PipelineSource`, `PipelineProcessor`, or
`PipelineSink`. Types flow through at compile time and a single OS thread
runs all of them on one current-thread tokio runtime. Each processor declares
`input_size()` and `output_size()`; `Some(n)` is checked by
`check_size_contract` (`src/pipeline.rs`), which panics if the neighbour
disagrees, while `None` means no guarantee. Struct names are often used as
the processor name.

### Silero VAD

Non-speech is filtered out before it reaches the model. Silero gives a speech
probability for every 512-sample chunk (`src/vad.rs`; its weights come
compiled into the bunsen crate, not from `model/`). `SileroVadScorer`
attaches that score to each chunk, and `VadFilter` turns scores into
segments with hysteresis: speech starts only after two consecutive chunks at
or above the start threshold and rising, and ends after two below the end
threshold and falling. A short run of non-speech from just before the start
is kept as pre-roll so words aren't clipped. Thresholds and pre-roll length
are arguments to `VadFilter::new`, set in `src/main.rs`.

### Sliding window chunking

Transcription quality drops at the borders of an input window, so the
pipeline feeds the model overlapping windows rather than disjoint cuts.
`SlidingWindowChunker` emits fixed-size windows that advance by the uncut
middle only, the window minus the left and right cut; `IpaProcessor` then
discards the border logits after inference, so only the middle of each
window is decoded. At the start of a speech segment the chunker prepends
enough silence to cover the left cut so the cut cannot swallow real speech,
and `SpeechEnd` right-pads with silence to finish the window. Sizes and
cuts come from `SlidingWindowConfig`, constructed in `src/main.rs`.
`src/bin/shift_tester.rs` measures how leading padding shifts the model's
output.

### dictionary.rs

`src/dictionary.rs` finds keywords: given the continuous IPA string the
model produced, it returns which words from `dictionary.json` occur in it
and how many bytes it consumed. It greedily matches the longest exact words
first, then runs leftover fragments through fuzzy matching scored by
phonetic similarity, with a threshold proportional to word length and a small
lookahead for length differences. `WordDetector` drives it: it buffers
transcribed text, clears the buffer when the gap between windows exceeds
300 ms, and trims whatever was consumed.

### Timestamping

Audio travels as `TimestampedVec<T>`, which is `Vec<(T, Duration)>`
(`src/timestamped_vec.rs`). Every sample carries its time since the UNIX
epoch, spaced one sample period apart. `AudioCapture` back-dates each
callback buffer to when recording of it started; `MemoryAudioSource` stamps
the whole file at load time, so file timestamps are only meaningful relative
to each other. `IpaProcessor` derives the start and end time of each
transcription from those sample timestamps plus the logit cut, and
`WordDetector` uses them to detect speech gaps.

## Tests

`src/dictionary.rs` has a bunch of tests for checking how it behaves on different inputs. Run after modifying the file.
