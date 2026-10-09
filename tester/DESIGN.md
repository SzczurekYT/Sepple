# tester — live pipeline debugger

Design document for the `tester` GUI tool: a separate binary that runs alongside the
live capture pipeline and visualizes, on one synchronized timeline, what every stage of
the processing pipeline is doing as it happens.

---

## 1. Overview and goals

Sepple's speech→IPA pipeline is built from processors (capture → chunking → Silero VAD
→ gating → sliding windows → IPA model → word detection). Today the intermediate
behavior of that pipeline is only observable through scattered `println!` debugging output.
There is no way to see, for one continuous stretch of time, how the raw audio, the VAD
decisions, the model input, and the detection results relate to each other.

`tester` closes that gap. It is a window that runs next to the normal pipeline and
displays eight strips on a single shared time axis, updated live (top to bottom):

1. raw audio waveform,
2. audio waveform after it passed through the Silero gate,
3. Silero speech score and the VAD gate decision,
4. the sliding-window chunks fed to the model (with overlap visible),
5. IPA tokens at the exact times they were assigned,
6. the WordDetector text buffer at each detection pass,
7. detected dictionary words positioned at the audio they matched,
8. detection latency per detected word.

**Goals for v1**

- Live visualization of all eight strips, synchronized, scrolling with the audio.
- Start/Stop control: Stop freezes the display and silences the audio flowing through
  the pipeline; Start resumes with a clean timeline.
- Full inspection of a stopped timeline: pan, zoom, select a time range on any strip,
  and export the selected audio for further offline analysis.
- Read-only instrumentation: the existing `sepple` CLI behaves exactly as before; all
  tester-only machinery lives in the new crate.

**Out of scope for v1:** live parameter tuning from the GUI, audio playback, and any
console output from the tester (the tester prints nothing).

This document is a design reference: it specifies the user-facing behavior and the
business logic first, and only gives a general overview of the implementation structure
(sections 4–8), enough to derive a detailed implementation plan from it.

---

## 2. UI/UX spec

### 2.1 General layout

A single window, default size ≈ 1280×800. From top to bottom:

- **Toolbar row** — Start / Stop buttons, current state indicator, zoom control,
  channel-overflow indicator, and the export group (source dropdown, Export button,
  preview text).
- **Eight strips**, stacked, all sharing the same horizontal time axis. Waveform strips
  are taller than event strips; heights are proportional, not equal.
- **Time axis** at the bottom, showing session-relative times (since the first
  recorded event of the session); the right edge is the live edge while running.

Default visible span is **10 seconds**, adjustable by zoom. All strips always show the
same time range — the timeline is the backbone of the tool: one thing happens at one
horizontal position on every strip.

```
+----------------------------------------------------------------------+
| [Start] [Stop]  state: running   zoom [---|---]   dropped events: 0  |
| source [Raw          v]  [Export...]  will export selected range     |
+----------------------------------------------------------------------+
| 1  Waveform raw      \______/\^^^^^^^\______/ \                      |
| 2  Waveform post-VAD          \^^^^^^^\__/                           |
| 3  Silero score+gate   ____/^^^^^^\____   thr 0.35   |gate: ▓▓▓▓▓▓  |
| 4  SW chunks     top  [===chunk n===]       [===chunk n+2===]        |
|                  bot       [===chunk n+1===]      [===chunk n+3===]  |
| 5  Tokens                [ˈ][k][æ][t]       [ˈ][h][ə][l][oʊ]        |
| 6  WD buffer       snapshots at each detection pass: ...             |
| 7  Words (audio span)     |hello|            |world|                 |
| 8  Latency                |hello|                                    |
+----------------------------------------------------------------------+
| time axis    <----------- 10 s window, live edge at right --------> |
+----------------------------------------------------------------------+
```

### 2.2 Controls and gestures

**Start / Stop.** The two buttons replace any notion of play/pause:

- **Stop** — two things happen at once:
  1. a processor inserted right after the audio source silences the stream (every
     sample becomes zero), so the rest of the pipeline keeps running but receives
     digital silence — VAD closes the gate naturally, no pipeline state is reset or
     lost, and no timestamps are skipped;
  2. the GUI freezes the timeline and stops recording: no new data is drawn, the view
     stops scrolling.
- **Start** — resumes recording and **resets the timeline**: all strips, history, and
  any active selection are cleared, and the view re-attaches to the live edge. Because
  everything is cleared, there is never a "gap" in the display — a stopped period is
  simply not part of the timeline.

**While running:** the time window is locked to the live edge (it scrolls with the
incoming audio). The zoom control still works; pan and selection gestures are inert.

**While stopped** (the inspection state):

| Gesture          | Action                                                     |
|------------------|------------------------------------------------------------|
| Mouse wheel      | Pan the timeline (all strips together)                     |
| Ctrl+Mouse wheel | Zoom the time axis (all strips together)                   |
| Right-drag       | Pan the timeline (all strips together)                     |
| Left-drag        | Draw a selection over a time range — works on any strip    |
| Left-click       | Clear the current selection                                |

**Selection** can be initiated on **any strip** — the time axis is shared, so the strip
the drag starts on is irrelevant. The selection is drawn as a shaded vertical band
across **all eight strips**, so the user immediately sees which tokens, buffer
snapshots, words and gate decisions fall inside the selected range. Selection is only
possible while stopped.

**State indicator** shows `running` / `stopped` at a glance.

**Overflow indicator** — the GUI never blocks the audio pipeline; if the GUI stalls and
events are dropped, a counter in the toolbar shows how many were lost (`dropped
events: N`), so the display is never silently wrong.

### 2.3 Export

The export group sits in the toolbar:

- **Source dropdown** — chooses which strip to export: `Raw`, `Post-VAD`, or
  `SW chunks`.
- **Preview text** next to the Export button updates live with the selection:
  - Raw → `will export selected range`
  - Post-VAD / SW chunks → `will export N chunks` (the number of files that will be
    written)
- **Export… button** — enabled only while stopped and a selection exists; opens a
  native save dialog, then writes the files.

All exports are **WAV, 16 kHz, mono, 16-bit PCM** — the project's standard audio
format, suitable for direct analysis.

Export semantics per source:

| Source      | Output                                                                                     |
|-------------|--------------------------------------------------------------------------------------------|
| Raw         | Exactly the selected samples of the raw waveform — one file, at the path chosen in the dialog. |
| Post-VAD    | The selected range split at gaps: every contiguous run of gate-passed audio becomes its own file (`name_0.wav`, `name_1.wav`, …). Gaps (rejected audio) are what create the split. |
| SW chunks   | One file per sliding-window chunk that intersects the selection (`name_0.wav`, `name_1.wav`, …) — full windows as the model received them, including overlap and padding. |

In the chunked modes the chosen path is a base name and files are always suffixed
`_0`, `_1`, … even when there is only one.

### 2.4 The strips

#### 2.4.1 Waveform raw (strip 1)

The unmodified audio as captured (or loaded from file), before any VAD processing. Drawn
as a standard min/max envelope waveform at full amplitude, and the reference waveform
against which everything below is read. Selections may be started on any strip
(section 2.2), but this is the waveform most selections will be made against.

#### 2.4.2 Waveform post-VAD (strip 2)

The audio that actually passed the gate, drawn in the same style as strip 1, at the same
scale. Regions where the gate was closed are simply empty — the visual difference
between strips 1 and 2 *is* the VAD's effect, at a glance. Includes pre-roll audio kept
by the gate so speech onsets are not clipped.

#### 2.4.3 Silero score + gate (strip 3)

Two overlaid signals, explaining why strips 1 and 2 differ:

- **Score curve** — the Silero speech probability per 512-sample chunk, drawn as a
  step curve on a 0–1 axis.
- **Threshold lines** — the configured start/end thresholds (0.35 / 0.35 in the
  current pipeline configuration), drawn as horizontal reference lines, so the reader
  can see why a chunk did or did not pass.
- **Gate band** — a shaded horizontal band shown while the gate is open (the
  hysteresis state of the filter after that chunk). Because of hysteresis the band is
  not derivable from the score alone; it reports the filter's actual decision.

#### 2.4.4 SW chunks (strip 4)

The 1-second windows handed to the IPA model, with their overlap made visible. Windows
advance by their uncut middle only, so consecutive chunks overlap in time — more data
than time — which is drawn with **alternating lanes**: chunk *n* on the top lane,
chunk *n+1* on the bottom lane, round-robin. With the current configuration (1000 ms
window, 700 ms advance) at most two windows overlap at any instant, so two half-height
lanes suffice; the lane count is derived from the configuration
(`ceil(window / advance)`), so the strip stays correct if the parameters change.

Each chunk is drawn at its true time span with its waveform, and its **cut borders are
shaded**: the left/right regions that `IpaProcessor` discards are visually distinct
from the uncut middle that becomes model input. This strip exists to debug exactly
those cuts — misaligned cuts show up here as speech falling into a shaded border.

#### 2.4.5 Tokens (strip 5)

Individual IPA tokens rendered as small labeled markers at their **exact assignment
time** (frame-accurate, derived from the model's CTC frames — not interpolated).
A token is one decoded symbol, which in IPA may consist of several UTF-8 codepoints
(e.g. affricates with combining ties); each such symbol still gets a single marker.

#### 2.4.6 WD buffer (strip 6)

A snapshot of the WordDetector's text buffer, labeled at the moment each detection pass
completes — i.e. every time an IPA text was appended, matched against the dictionary,
and trimmed. Reading this strip left-to-right shows exactly how the buffer accreted and
drained into detected words (including clears caused by speech gaps). Long snapshots
are displayed truncated to their tail — the recent end is what matters.

#### 2.4.7 Words — audio span (strip 7)

Each detected dictionary word is drawn as a labeled bar spanning **the audio it
matched** — the time range of the buffer text that produced it. This answers "where in
the speech was this word".

#### 2.4.8 Latency (strip 8)

A stem plot: one stem per detected word, placed at the **moment detection fired**, with
height = `detection time − end of the word's audio span` (seconds). This answers "how
late did we learn it" — model window delay, buffering, and dictionary matching lag all
show up as taller stems. Y-axis auto-scales to the visible stems.

---

## 3. Strips data sources

All instrumentation is **additive and external**: taps are pass-through processors that
clone data to a side channel and forward it unchanged; wrappers are processors that hold
the original processor, delegate every call to it, and additionally read its state.
Everything listed here lives in the `tester` crate (section 5). The tester assembles
its own pipeline, mirroring `sepple pipeline` with the additions below (AudioLogger is
omitted, and there is no terminal value printer — see the notes below the table).

```
AudioCapture | MemoryAudioSource
  → SilenceGate                            (Start/Stop: zeroes samples)
  → AudioChunker
        │ tap ────────────────────────────► strip 1  raw waveform
  → SileroVadScorer
  → InstrumentedVadFilter                  (wraps VadFilter; reads score + gate)
        │ tap ────────────────────────────► strip 2  post-VAD waveform
  → SlidingWindowChunker
        │ tap ────────────────────────────► strip 4  SW chunks
  → IpaProcessor
        │ tap ────────────────────────────► strip 5  tokens
  → InstrumentedWordDetector               (wraps WordDetector; buffer snapshots)
        │                                   ► strip 6  WD buffer
  → Tap (terminal)                         ► strips 7+8  words / latency
```

**Strip-by-strip:**

| Strip | Comes from | Event payload (essence) |
|-------|------------|--------------------------|
| 1 raw waveform | tap after `AudioChunker` | audio chunk with per-sample timestamps (post-SilenceGate, pre-VAD) |
| 2 post-VAD waveform | tap after `VadFilter` | emitted data chunks with their original sample timestamps (pre-roll included; `SpeechEnd` carries no audio) |
| 3 score + gate | the `VadFilter` wrapper, on each of its inputs | chunk time span, score, gate state after processing that chunk |
| 4 SW chunks | tap after `SlidingWindowChunker` | full window: samples + start timestamp (cut widths known from the pipeline configuration) |
| 5 tokens | tap after `IpaProcessor` | text + window start/end + per-symbol strings with exact times |
| 6 WD buffer | the `WordDetector` wrapper, after each delegated call | timestamp + buffer contents post-pass |
| 7/8 words & latency | terminal `Tap` (as sink) after `WordDetector` | word + matched audio span + detection timestamp |

Notes:

- The scorer itself stays unwrapped: the wrapper around `VadFilter` sees the same
  `(chunk, score)` pairs on its input, so one wrapper feeds both the score curve and
  the gate band.
- The gate state and the buffer contents are private fields of their processors;
  the wrappers read them through the small additive accessors listed in section 4.
- A `Tap` at the end of the pipeline — where it implements the sink role instead of the
  processor role (section 5) — replaces `ValuePrinter`: it drains the final output into
  the GUI and discards it, so the pipeline has a consumer (it must, or it would stall)
  while the tester prints nothing.

---

## 4. sepple library changes

Only additive changes; existing behavior and the existing CLI output stay
byte-identical:

1. **`VadFilter::is_talking()`** — getter exposing the gate's hysteresis state, for
   the gate band (strip 3).
2. **`WordDetector::buffer()`** — getter exposing the current text buffer, for the
   snapshot strip (strip 6).
3. **`WordDetector` output type becomes `DetectedWord`** — `{ word, audio_span,
   detected_at }`, each field with a distinct role:
   - `word` — the matched dictionary word itself: what downstream consumers see, and
     what the `Display` prints (only the word), keeping `sepple pipeline`'s
     `ValuePrinter` output byte-identical;
   - `audio_span` — the time range in the source audio covered by the buffer text that
     matched; positions the word bar in strip 7 and provides the latency baseline;
   - `detected_at` — the wall-clock moment the detection fired; positions the stem in
     strip 8, whose height is `detected_at − audio_span.end`.
4. **`TimestampedText` gains per-symbol times** — the decoded symbol strings (IPA
   graphemes, possibly multi-codepoint) paired with their exact frame-derived times,
   produced inside `IpaProcessor` from the already existing
   `greedy_ctc_decode_with_indexes`. The existing fields and `Display` are untouched.

---

## 5. tester crate structure (general)

`tester/` is a new workspace member (like `trainer`: path dependency on `sepple`,
forwarding the `wgpu` feature), adding `eframe`/`egui_plot` for the window and `rfd`
for the native file dialog.

CLI: `tester [-f FILE]` — live microphone by default, `-f` feeds a WAV file through the
same pipeline (reproducible runs). No other flags; the tester prints nothing.

Responsibilities, by module:

- **pipeline** — assembles the pipeline exactly as in section 3 from the same
  configuration values `sepple pipeline` uses (window sizes, cuts, thresholds,
  dictionary), minus AudioLogger, plus SilenceGate, taps, wrappers, and the terminal
  tap; owns the lifecycle hooks (section 6).
- **instrumentation** — `SilenceGate`, the generic tap, and the two wrappers. The tap is
  a single type `Tap<T>` that clones every value passing through it to a GUI channel
  and knows nothing about the GUI itself. One type covers both placements:
  - mid-pipeline it implements `PipelineProcessor` — clone the value to the channel,
    then forward it to the next stage unchanged (a pure observer);
  - at the end of the pipeline it implements `PipelineSink` — clone to the channel,
    then drain and discard (the pipeline's terminal consumer).
  The wrappers hold the processor they instrument, delegate every trait call to it
  unchanged, and additionally read its state (via the additive accessors from
  section 4) into the same kind of GUI channel.
- **gui** — the eframe application: toolbar, the eight strips, the shared time-axis
  state, the session history store (section 6), selection, and export (section 2.3).

---

## 6. Threading, transport and lifecycle (general)

**Two worlds.** The pipeline runs on its own OS threads with its own runtimes (existing
machinery); the GUI owns the main thread's event loop. They communicate only through
per-feed channels.

**Transport.** Every instrumentation feed has its own bounded channel — a feed is one
tap or wrapper output, not one strip: a single feed may serve several strips (detected
words, for instance, come from one channel that feeds both the words strip and the
latency strip). Taps push with a non-blocking try-send — the pipeline is never blocked
or slowed by the GUI; on overflow the event is dropped and counted, and the toolbar
counter is incremented. The GUI drains every feed's channel each frame into the shared
app-state history store, which is single-owner and feeds both rendering and export.

**History / retention.** Waveform tracks keep **full raw samples for the whole
session** (unbounded; ≈ 23 MB/hour per track, and a session ends at the next Start
anyway) — required for selection/export. Display envelopes are derived from those raw
samples for the visible range only. Event tracks (scores, tokens, snapshots, words)
are tiny and kept whole.

**Timeline origin.** All timestamps the GUI stores are session-relative: ingestion
re-bases every event by the origin, defined as the stamp of the first event accepted
after Start; Start (and its clear) resets the origin, so a new session always begins
at 0. Live capture, file playback and mockup data all flow through the same rule.

**Start/Stop path.** The GUI toggles a shared signal read by `SilenceGate`; the GUI-side
freeze/clear is purely local. On Start, after clearing, any in-flight events stamped
before the Start instant are discarded while draining, so pre-Stop leftovers can never
appear on the fresh timeline.

**Lifecycle.** The pipeline is built with the existing "no consumer" constructor, which
hands back the final receiver plus a pipeline handle; the terminal `Tap`'s sink future
runs on a background thread. Closing the window drops the handle and the terminal
`Tap`'s thread — the process exit reclaims everything, so there is no cancel/join.

---

## 7. Rendering approach (general)

One `egui_plot` per strip, all driven by a single shared x-view state: running locks it
to the live edge (zoom still applied), stopped lets it be panned/zoomed (section 2.2).

- **Waveforms** are drawn as min/max envelope polylines: raw samples of the visible
  range are bucketed per pixel column, so point counts are bounded by window width
  regardless of zoom or session length — the main performance guarantee. The same
  buckets render strips 1, 2, and the waveform inside each lane of strip 4.
- **Strip 3** is a step polyline (score), two horizontal reference lines (thresholds),
  and a shaded band region (gate).
- **Strip 4** alternates chunks between lanes round-robin and overlays cut-border
  shading from the pipeline's window configuration.
- **Strips 5–8** are marker/label/stem primitives positioned from event timestamps.
- The selection band is one more region drawn identically in every strip.

Performance note: eight plots at interactive frame rates are expected to be fine given
the bounded point counts; if plot overhead ever becomes the bottleneck, the waveform
lanes can move to direct shape painting without changing any design decision.

---

## 8. Edge cases and behaviors

- **File mode (`-f`)** — identical features; file timestamps are only meaningful
  relative to each other (as elsewhere in the project), so the timeline is read
  relative. The source is piped at real-time speed, so file mode's clock coincides
  with the wall clock exactly like live capture; the view follows the live edge. When
  the file is exhausted the pipeline completes, the GUI stays open showing the final
  timeline, and Start/Stop remain operable (no new data will arrive).
- **Stopped for a long time** — the pipeline consumes silence the whole time; nothing
  is recorded, so there is nothing to catch up on; Start clears and continues.
- **Empty results** — windows that decode to no text, or passes that find no words,
  simply produce no events; strips stay sparse rather than erroring.
- **Export edge cases** — Export is disabled unless stopped with an active selection;
  starting clears the selection. A selection with no data in the chosen source exports
  zero files (the preview already shows `N = 0`).
- **Overflow** — dropped events are counted per the toolbar indicator; the audio
  pipeline itself is never affected.
