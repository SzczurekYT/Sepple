# tester — implementation plan

High-level steps to implement `DESIGN.md`, in build order: the GUI first, driven by
mockup data; then the `sepple` library changes for sourcing; then transport and full
data sending. Verification is done by the user between phases, not part of this plan.

**Convention — module files:** we don't use `mod.rs` — high-level code is in
`module.rs`, and details / parts it uses land in `module/something.rs`.

# Phase 1 — GUI built on mockup data (no transport yet)

## 1. Flesh out the `tester` crate skeleton

Add the `sepple` path dependency with `wgpu` feature forwarding, a flat clap CLI with
only an optional `-f/--file`, and the three modules `pipeline` / `instrumentation` /
`gui`. The binary parses its arguments and immediately opens an empty **"sepple
tester"** window (≈1280×800); it prints nothing on success and only emits a stderr
diagnostic on a fatal failure.

### Dependencies and features

`tester/Cargo.toml` gains `clap` and the `sepple` path dependency (as in `trainer`),
plus a `wgpu` feature for `sepple` backend forwarding, and the three GUI deps:

```toml
[features]
wgpu = ["sepple/wgpu"]

[dependencies]
clap.workspace = true
sepple = { path = "../" }
eframe = "0.36.2"       # default features kept — wgpu renderer
egui_plot = "0.37.0"    # first used in point 5
rfd = "0.17.2"          # first used in point 14
```

`sepple`, `egui_plot` and `rfd` are intentionally unused in point 1 — they arrive with
their respective points. The `wgpu` feature only swaps `SeppleBackend` between `Flex`
and `Wgpu`; the GUI renderer is unaffected by it.

### CLI

A flat derive struct in `main.rs`, no subcommands and no `-v`:

```rust
#[derive(Parser)]
#[command(version, about = "live pipeline debugger", long_about = None)]
struct Cli {
    /// Feed a WAV file instead of the live microphone
    #[arg(short = 'f', long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = gui::run(cli.file) {
        eprintln!("tester failed: {err:?}");
        std::process::exit(1);
    }
}
```

`-f` and `--file` both work; `-h/--help` and `--version` are clap-generated.

### Output and error policy

- On success the process writes nothing to stdout, ever.
- On fatal failure (currently only eframe window/init errors) a multi-line
  `Debug`-formatted diagnostic goes to stderr and the process exits 1.
- clap owns its own behavior: usage errors → stderr + exit 2, `--help`/`--version` →
  stdout.
- Third-party runtime noise is tolerated (cpal stream `eprintln!`, panic messages, the
  phase-3 model-download progress bar).
- `set_debug` is never called (there is no `-v` flag), so all of sepple's
  `debug_enabled()` `println!`s stay off.

### Module layout

```
tester/src/
├── main.rs                 # Cli + main
├── pipeline.rs             # high-level entry — assembly lands in point 20
├── instrumentation.rs      # high-level entry — SilenceGate/Tap/wrappers in point 19
└── gui.rs                  # pub fn run + empty eframe App — shell in point 3
```

The `module/` directories holding each module's parts (`gui/strips.rs`,
`gui/export.rs`, `pipeline/taps.rs`, …) appear when their first part lands in a later
point; high-level code always stays in the sibling `module.rs`.

### Window

`gui::run(file: Option<PathBuf>) -> Result<(), eframe::Error>` opens the window
immediately: title **"sepple tester"**, inner size 1280×800 per §2.1, and an
`eframe::App` whose `ui` draws nothing (a plain clear-color window) while storing
`file` in its struct for later points. Closing the window ends the process — there is
no pipeline to cancel yet.

## 2. Define the timeline store (`TimelineData`) with mockup data

`gui/timeline_data.rs` defines `TimelineData` — the single owner of all eight strips'
data — and `gui/mockup.rs` provides `TimelineData::mockup()`, a pre-filled ~12 s
consistent scenario. Normal operation constructs it empty and feeds append to it; the
GUI renders purely from this struct, no channels yet.

### Modules and responsibilities

- **`gui/timeline_data.rs`** — `TimelineData`, `SessionConfig`, event structs,
  ingest/clear API, raw-as-segments bridge.
- **`gui/mockup.rs`** — `impl TimelineData { pub fn mockup() -> Self }` only;
  scaffolding, removed once real feeds replace it.
- **`gui.rs`** — App gains a `timeline: TimelineData` field initialized with
  `mockup()`; nothing renders from it yet.

`TimelineData` is **session data only** (tracks + config + append/clear). View/zoom,
selection, running flag and dropped counter belong to the App itself.

### Time model

- Every stored timestamp is a **session-relative `Duration`** — what the GUI displays
  everywhere.
- Re-basing happens **at ingestion**: `rebased(stamp)` sets `origin` to the first
  accepted stamp and yields `stamp − origin` thereafter; `clear()` empties tracks and
  resets `origin` to `None`. One rule covers live (epoch), file (load-time epoch) and
  mockup (already 0-based).
- All events share **one clock** — the audio clock, which the real-time piping fix
  makes coincide with wall time in both modes; that keeps latency
  (`detected_at − span.end`) positive and strips 3–8 aligned with the waveforms.
- Only the mockup path exists now (fields written 0-based); `rebased()` gets exercised
  when the feed drain lands.

### `SessionConfig`

```rust
pub struct SessionConfig {
    pub start_threshold: f32,   // 0.35
    pub end_threshold: f32,     // 0.35
    pub window_size: Duration,  // 1000 ms
    pub cut_left: Duration,     // 150 ms
    pub cut_right: Duration,    // 150 ms
}
impl SessionConfig {
    pub fn advance(&self) -> Duration { self.window_size - self.cut_left - self.cut_right } // 700 ms
}
```

Lives **inside** `TimelineData`: strips 3/4 render thresholds, cut borders and lane
count from it. `Default` uses the numbers above.

### Tracks and event structs

```rust
pub struct TimelineData {
    pub config: SessionConfig,
    origin: Option<Duration>,           // private; set by first rebased() call
    pub raw: RawAudio,                  // strip 1
    pub post_vad: Vec<Segment>,         // strip 2 — one per gate run, gaps = gate closed
    pub scores: Vec<ScoreEvent>,        // strip 3
    pub chunks: Vec<ChunkEvent>,        // strip 4
    pub tokens: Vec<TokenEvent>,        // strip 5 — flat, one per symbol
    pub snapshots: Vec<BufferSnapshot>, // strip 6
    pub words: Vec<WordEvent>,          // strips 7 + 8 — one feed serves both
}

pub struct RawAudio       { pub start: Duration, pub samples: Vec<f32> }  // dense, continuous
pub struct Segment        { pub start: Duration, pub samples: Vec<f32> }
pub struct ScoreEvent     { pub span: Range<Duration>, pub score: f32, pub gate_open: bool }
pub struct ChunkEvent     { pub start: Duration, pub samples: Vec<f32> }
pub struct TokenEvent     { pub time: Duration, pub symbol: String }
pub struct BufferSnapshot { pub at: Duration, pub text: String }
pub struct WordEvent      { pub word: String, pub span: Range<Duration>, pub detected_at: Duration }
```

- Event types are **tester-local** — no sepple types; payloads get converted at the
  ingest boundary.
- Spans are **`Range<Duration>`** everywhere.
- Raw is **dense** (`f32` + `start`, ≈23 MB/h); post-VAD is a **segment list**.
  `RawAudio::iter()` yields a single `(start, &[f32])` pair so strips 1/2/4 chain it
  with `post_vad` into one uniform code path.
- API: per-track `push_*`, `rebased(stamp)`, `clear()` (keeps `config`), `Default` =
  empty tracks + default config.

### Mockup scenario (~12 s, deterministic, no RNG)

- **Utterances:** U1 `0.8–3.4`, U2 `4.6–7.9`, U3 `9.1–11.6` — gaps > 300 ms so buffer
  clears show in snapshots.
- **Strip 1:** continuous 12 s (192k samples ≈ 768 KB), speech-like envelope inside
  utterances, near-silence between.
- **Strip 3:** score per 32 ms (512-sample chunk); floor ~0.05, peaks 0.6–0.95;
  `gate_open` computed with the real hysteresis rule (2 consecutive ≥0.35 rising →
  open, 2 consecutive <0.35 falling → closed).
- **Strip 2:** exactly the gate-open runs + 320 ms pre-roll (10 chunks), consistent
  with the same flags.
- **Strip 4:** 1000 ms windows starting ~0.19 s before each gate-open, stepping 700 ms
  over the utterance; samples sliced from the raw array so waveforms match strip 1
  exactly; cut borders from `config`.
- **Strip 5:** dictionary IPA symbols at ~35–50 ms spacing, inside window middles
  (≥150 ms from cuts).
- **Strip 6:** one snapshot per window's token batch: growth (`ˈun` → `ˈunvaksɒm`),
  post-match trim (`""`), post-gap clear.
- **Strips 7/8:** real `dictionary.json` words — `ˈunvaksɒm` (U1), `vəˈluɡoː` +
  `plɒka` (U2), `lirɔ` (U3); `span` = matched tokens' audio range,
  `detected_at = span.end + 0.15…0.4 s` → positive stems, all within 12 s.

## 3. Build the GUI shell

`gui.rs` owns the App and its `ui()`; `gui/toolbar.rs`, `gui/plots.rs`,
`gui/time_axis.rs` render the parts. The shell shows the full toolbar, eight empty
labelled strip plots, and a time axis, all on a hardcoded 0–10 s range; strip data
arrives with the per-strip points.

### Module layout

- **`gui.rs`** — App struct, `ui()` orchestration, `start()`/`stop()`
- **`gui/toolbar.rs`** — toolbar row
- **`gui/plots.rs`** — label column + eight plots
- **`gui/time_axis.rs`** — bottom time axis

### App struct

```rust
pub struct App {
    file: Option<PathBuf>,
    timeline: TimelineData,
    running: bool,
    dropped: usize,
}

impl App {
    fn start(&mut self) { self.running = true; }  // extended when Start/Stop lands
    fn stop(&mut self) { self.running = false; }
}
```

### Toolbar (`gui/toolbar.rs`)

`egui::Panel::top("toolbar").show(ui, |ui| ...)` with `ui.horizontal`:

- Start / Stop buttons → `self.start()` / `self.stop()`
- state label: `"running"` / `"stopped"`
- overflow: `ui.label(format!("dropped events: {}", self.dropped))`
- export group: `ComboBox` (Raw / Post-VAD / SW chunks), disabled `Export...` button,
  preview label — inert placeholders

### Plots (`gui/plots.rs`)

- Fixed-width left label column (~120 px) + plot column filling the rest; the time axis
  spans only the plot column so x positions align across all strips.
- Heights: flexbox weights `[4, 4, 2, 4, 1, 1, 1, 2]` (total 19) of the available height
  below the toolbar.
- Each plot: `Plot::new("stripN").height(h).auto_bounds(false.into()).show(ui, |plot_ui| plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, y_min], [10.0, y_max])))`
  — x hardcoded 0–10 s; y per strip type (waveforms −1..1, score 0..1, events 0..1);
  empty until the per-strip points.
- Labels from the figure: `1 Waveform raw`, `2 Waveform post-VAD`,
  `3 Silero score+gate`, `4 SW chunks`, `5 Tokens`, `6 WD buffer`,
  `7 Words (audio span)`, `8 Latency`.

### Time axis (`gui/time_axis.rs`)

- Thin `Plot` (~24 px) under the plot column, x-axis only (`.show_axes([true, false])`,
  no grid/background), same 0–10 range.
- Adaptive ticks: spacing chosen from `[0.1, 0.2, 0.5, 1, 2, 5, 10, 15, 30, 60]` so ~5–8
  labels fit; whole numbers render without `.0` (`0`, `1`, `10`), fractional with one
  decimal (`0.5`, `2.5`).

### Layout

- Fixed layout, no scroll; window resizable, heights follow the weights.

## 4. Implement the shared x-view behavior

`gui.rs` gains `ViewState { center, span }` and the live-edge lock; all nine plots link
their x-axis via `link_axis` and use built-in pan/zoom; the only manual gesture left is
left-drag selection. While running, the view is locked to the live edge (raw track's last
sample) and built-in gestures are disabled; while stopped, plain wheel pans, ctrl+wheel
zooms, right-drag pans.

### Code map — where everything lands

| File | Item | Change |
|---|---|---|
| `gui.rs` | `struct ViewState { center: f64, span: f64 }` | new |
| `gui.rs` | `App.view: ViewState` | new field |
| `gui.rs` | `App::ui()` | frame loop: live-edge lock → push view → show plots → mirror bounds |
| `gui.rs` | `fn live_edge(raw: &RawAudio) -> f64` | raw track's last sample time |
| `gui/toolbar.rs` | zoom slider | introduced here; binds to `app.view.span` |
| `gui/plots.rs` | `Plots::render(app, ui)` | 8 strip plots; first-built gets the x push; each sets y per type; all get `link_axis` + `allow_*` |
| `gui/time_axis.rs` | `TimeAxis::render(app, ui) -> PlotResponse<()>` | axis plot, same config; built last; its response feeds the mirror |

### ViewState and the frame loop (`gui.rs`)

```rust
pub struct ViewState {
    pub center: f64,
    pub span: f64,
}
```

- Initial: `center = 5.0`, `span = 10.0` (replaces point 3's hardcoded 0–10 s).
- Frame loop in `App::ui()`:
  1. `live_edge = raw.start + sepple::units::sample_count_to_duration(raw.samples.len())`;
     if `running`, `view.center = live_edge − span / 2`.
  2. Push `view` x-bounds onto the **first-built** plot
     (`set_plot_bounds_x(center − span/2 ..= center + span/2)`).
  3. Show all nine plots (built-in gestures may modify bounds).
  4. Mirror: `view.center`/`span` ← last plot's `PlotResponse.transform.bounds()` (the
     time axis, built last). The mirror includes any built-in pan/zoom, so re-pushing is
     a no-op — the loop is stable.
- The zoom slider binds to `view.span`; the live-edge lock overwrites `view.center` while
  running.

### Live edge (`gui.rs`)

- `live_edge = raw.start + sepple::units::sample_count_to_duration(raw.samples.len())` —
  the raw track's last sample time.
- While running: `view.center = live_edge − span / 2` every frame → right edge is the
  live edge; the slider still works (`span` independent).
- While stopped: no lock; the view stays where the user left it.

### Plot configuration (`gui/plots.rs`, `gui/time_axis.rs`)

```rust
Plot::new("stripN")
    .height(h)
    .link_axis("time_link", Vec2b::new(true, false))    // share x only
    .auto_bounds(false.into())
    .allow_drag(Vec2b::new(!running, false))            // right-drag pan, x only
    .allow_scroll(Vec2b::new(!running, false))          // plain wheel pan, x only
    .allow_zoom(Vec2b::new(!running, false))            // ctrl+wheel zoom, x only
    .pan_pointer_button(egui::PointerButton::Secondary)
    .show(ui, |plot_ui| { /* y bounds per strip type */ })
```

- `!running` disables all three built-in gestures while running (only the slider works);
  while stopped they're x-only.
- The first-built plot receives the `view` x-push; the link group propagates it to all
  others same-frame. A pan started on a later-built plot leaves earlier-built plots one
  frame behind (~16 ms, imperceptible).
- Y bounds set per strip type (waveforms −1..1, score 0..1, events 0..1); never touched
  by gestures — every gesture axis is x-only.
- The time axis plot (`gui/time_axis.rs`) is built last with the same config; its
  `PlotResponse` feeds the mirror.
- The common Plot config is wrapped in a `base_plot(id, running, height)` helper in
  `gui/plots.rs`, so each strip render function only adds its strip-specific items.

### Gestures

- **Plain wheel** → pan (`allow_scroll`, x-only) — built-in.
- **Ctrl+wheel** → zoom around cursor (`allow_zoom`, x-only) — built-in.
- **Right-drag** → pan (`pan_pointer_button(Secondary)`) — built-in.
- **Left-drag** → selection — manual (left-drag is free because pan is on Secondary).
- **While running**: all built-in gestures disabled; only the zoom slider works.

## 5. Strip 1 — raw waveform

`gui/waveform.rs` introduces the shared envelope helper; `gui/plots.rs` renders strip 1
as a filled envelope. The helper buckets the visible raw samples into per-pixel-column
(min, max) and returns them as three slices for `FilledArea`.

### Envelope helper (`gui/waveform.rs`)

```rust
pub struct Envelope {
    pub xs: Vec<f64>,
    pub ys_min: Vec<f64>,
    pub ys_max: Vec<f64>,
}

pub fn envelope(
    start: Duration,
    samples: &[f32],
    x_min: f64,
    x_max: f64,
    width_px: f32,
) -> Envelope { ... }

pub fn draw<'a>(
    plot_ui: &mut PlotUi,
    segments: impl Iterator<Item = (Duration, &'a [f32])>,
    x_min: f64,
    x_max: f64,
    name_prefix: &str,
    y_offset: f64,
    y_scale: f64,
    fill: Color32,
    stroke: Stroke,
) {
    let width_px = plot_ui.response().rect.width();
    for (i, (start, samples)) in segments.enumerate() {
        let mut env = envelope(start, samples, x_min, x_max, width_px);
        if env.xs.is_empty() {
            continue;
        }
        for y in env.ys_min.iter_mut().chain(env.ys_max.iter_mut()) {
            *y = y_offset + y_scale * *y;
        }
        plot_ui.add(
            FilledArea::new(&format!("{name_prefix}_{i}"), &env.xs, &env.ys_min, &env.ys_max)
                .fill_color(fill)
                .stroke(stroke),
        );
    }
}
```

- `width_px` = plot's screen-rect width → one bucket per pixel column; point count
  bounded by window width regardless of zoom or session length (DESIGN §7).
- Sample times are uniform: `t_i = start + i * (1 / SAMPLE_RATE)`. Compute the index
  range for `[x_min, x_max]` arithmetically (`i_start..i_end`) and iterate only that
  slice — O(visible samples) per frame, so the unbounded raw track stays cheap.
- Per column, track min/max; emit `(x_center, min, max)` for non-empty columns into
  `xs`/`ys_min`/`ys_max`.
- Empty result if `x_max <= x_min`, `width_px <= 0`, or the index range is empty.
- `draw` wraps `envelope` + `FilledArea` for a set of segments; `y_offset`/`y_scale`
  transform the envelope vertically (strips 1/2 use `0.0, 1.0`, strip 4 uses per-chunk
  lane transforms).

### `render_raw_waveform` (`gui/plots.rs`)

Creates the plot via `base_plot("strip1", running, h)` and calls `waveform::draw` with the
raw segment at `0.0, 1.0`. Y bounds (−1..1) from the per-strip config; semi-transparent
fill + solid stroke.

## 6. Strip 2 — post-VAD waveform

`gui/plots.rs` renders strip 2 by calling `waveform::draw` with the `post_vad` segments —
same style and scale as strip 1. Gate-closed regions have no segment, so they render
empty; the visual difference between strips 1 and 2 is the VAD's effect.

### `render_post_vad_waveform` (`gui/plots.rs`)

Creates the plot via `base_plot` and calls `waveform::draw` with the `post_vad` segments at
`0.0, 1.0`. Gaps between segments render empty; same styling as strip 1.

## 7. Strip 3 — Silero score + gate

`gui/plots.rs` renders strip 3 in three layers (back to front): a full-height shaded band
marking gate-open runs, a step polyline of per-chunk scores, and horizontal threshold
reference lines.

### `render_vad_state` (`gui/plots.rs`)

Creates the plot via `base_plot` and draws three layers (back to front): gate band
(full-height `FilledArea` per gate-open run), score step polyline (staircase of
`(span.start, score)`/`(span.end, score)` points), threshold `HLine`s at the configured
start/end thresholds. Y bounds (0..1) from the per-strip config; each layer gets a
distinct color.

## 8. Strip 4 — SW chunks

`gui/plots.rs` renders strip 4: each chunk is drawn at its true time span in a
round-robin lane, with its waveform (via `waveform::draw`, y-transformed to the lane) and
its cut borders shaded from the window config.

### `render_sw_chunks` (`gui/plots.rs`)

Creates the plot via `base_plot` and, per chunk (clipped to the visible range), draws
cut-border shading (behind) + waveform via `waveform::draw` with the lane transform. Lane
count `ceil(window/advance)`; chunk `n` → lane `n % lane_count`; lane `i` of `L` maps to
`y_offset = 1 − (2i+1)/L`, `y_scale = 1/L`. Chunks overlap, so lanes and shading stack.
Y bounds (−1..1) from the per-strip config; lanes divide it.

## 9. Strip 5 — tokens

`gui/plots.rs` renders strip 5 via `render_tokens`: each token is a small point at its
exact assignment time with its IPA symbol as a label above; when tokens are too dense for
labels to be legible, it renders points only.

### `render_tokens` (`gui/plots.rs`)

- **Markers** — `Points` at `(time, 0.5)` for each visible token, small radius.
- **Labels** — `Text` at `(time, 0.5)` with the symbol, anchored above (`CENTER_BOTTOM`).
  A multi-codepoint grapheme uses its symbol string as one label (single marker).
- **Density** — compute the minimum consecutive gap in pixels (`gap_px = Δt · width_px /
  span`); if it drops below a legibility threshold (~24 px for Small-font IPA), render
  points only.
- Y bounds (0..1) from the per-strip config.

```rust
Plot::new("strip5").height(h).link_axis(/* ... */).show(ui, |plot_ui| {
    let (x_min, x_max) = view.x_range();
    let width_px = plot_ui.response().rect.width();
    let px_per_sec = width_px as f64 / view.span.max(1e-6);
    let times: Vec<f64> = timeline.tokens.iter()
        .map(|t| t.time.as_secs_f64())
        .filter(|&t| t >= x_min && t <= x_max)
        .collect();
    plot_ui.points(
        Points::new("strip5_pts", times.iter().map(|&t| [t, 0.5]).collect())
            .radius(2.0).color(TOKEN),
    );
    let min_gap_px = times.windows(2)
        .map(|w| (w[1] - w[0]) * px_per_sec)
        .fold(f64::INFINITY, f64::min);
    if min_gap_px >= LABEL_MIN_GAP {
        for (i, tok) in timeline.tokens.iter().enumerate() {
            let t = tok.time.as_secs_f64();
            if t < x_min || t > x_max { continue; }
            plot_ui.text(
                Text::new(&format!("strip5_lbl_{i}"), PlotPoint::new(t, 0.5), &tok.symbol)
                    .color(TOKEN)
                    .anchor(Align2::CENTER_BOTTOM),
            );
        }
    }
});
```

## 10. Strip 6 — WD buffer

`gui/plots.rs` renders strip 6 via `render_wd_buffer`: each detection pass is a marker at
its completion time with the buffer text (after append/match/trim) as a label; long text
is truncated to its tail, and clears show as "∅".

### `render_wd_buffer` (`gui/plots.rs`)

- **Markers** — `Points` at `(at, 0.5)` for each visible snapshot.
- **Labels** — `Text` at `(at, 0.5)` with the buffer text. Empty buffer (clear) → "∅".
  Long text truncated to its tail (~24 graphemes) with a "…" prefix.
- Snapshots are sparse (one per detection pass) and the buffer is usually trimmed short,
  so labels rarely overlap; truncation handles the long case.
- The `at` time is the audio head when the pass completed (single audio clock).
- Y bounds (0..1) from the per-strip config.

```rust
Plot::new("strip6").height(h).link_axis(/* ... */).show(ui, |plot_ui| {
    let (x_min, x_max) = view.x_range();
    let visible: Vec<_> = timeline.snapshots.iter()
        .filter(|s| s.at.as_secs_f64() >= x_min && s.at.as_secs_f64() <= x_max)
        .collect();
    plot_ui.points(
        Points::new("strip6_pts", visible.iter().map(|s| [s.at.as_secs_f64(), 0.5]).collect())
            .radius(2.0).color(BUFFER),
    );
    for (i, s) in visible.iter().enumerate() {
        let label = if s.text.is_empty() {
            "∅".to_string()
        } else {
            truncate_tail(&s.text, 24)
        };
        plot_ui.text(
            Text::new(&format!("strip6_lbl_{i}"), PlotPoint::new(s.at.as_secs_f64(), 0.5), label)
                .color(BUFFER),
        );
    }
});
```

## 11. Strip 7 — words

`gui/plots.rs` renders strip 7 via `render_words`: each detected word is a semi-transparent
bar tinting the background across its audio range, with the word as a centered label.

### `render_words` (`gui/plots.rs`)

- **Bars** — a semi-transparent `FilledArea` rectangle at y = 0.5 (e.g. y ∈ [0.35, 0.65])
  spanning `[span.start, span.end]` for each visible word — tints the background rather
  than drawing a solid bar.
- **Labels** — `Text` at the bar's center `((start+end)/2, 0.5)` with the word.
- Word spans are sequential and non-overlapping, so bars form a single row.
- Y bounds (0..1) from the per-strip config.

```rust
Plot::new("strip7").height(h).link_axis(/* ... */).show(ui, |plot_ui| {
    let (x_min, x_max) = view.x_range();
    for (i, w) in timeline.words.iter().enumerate() {
        let (start, end) = (w.span.start.as_secs_f64(), w.span.end.as_secs_f64());
        if end < x_min || start > x_max { continue; }
        plot_ui.add(
            FilledArea::new(&format!("strip7_bar_{i}"), &[start, end], &[0.35, 0.35], &[0.65, 0.65])
                .fill_color(WORD_BAR),
        );
        plot_ui.text(
            Text::new(&format!("strip7_lbl_{i}"), PlotPoint::new((start + end) / 2.0, 0.5), &w.word)
                .color(WORD_TEXT),
        );
    }
});
```

## 12. Strip 8 — latency

`gui/plots.rs` renders strip 8 via `render_latency`: one stem per word at its detection
moment, height = `detected_at − audio_span.end`, y-axis auto-scaled to the visible stems.

### `render_latency` (`gui/plots.rs`)

- **Stems** — `Points::stems(0.0)` at `(detected_at, detected_at − span.end)` for each
  visible word — a vertical line from the height down to y = 0.
- **Y bounds** — auto-scaled: `y_min = 0`, `y_max = max(visible heights) × 1.2` (with a
  minimum, e.g. 0.5 s, so a single short stem doesn't collapse the axis). Computed each
  frame from the visible stems.
- **No labels** — the stems are self-explanatory (position = detection time, height =
  latency).
- X range from the view; stems filtered to the visible range.

```rust
Plot::new("strip8").height(h).link_axis(/* ... */).show(ui, |plot_ui| {
    let (x_min, x_max) = view.x_range();
    let visible: Vec<(f64, f64)> = timeline.words.iter()
        .map(|w| (w.detected_at.as_secs_f64(), w.detected_at.saturating_sub(w.span.end).as_secs_f64()))
        .filter(|(x, _)| *x >= x_min && *x <= x_max)
        .collect();
    let y_max = visible.iter().map(|(_, h)| *h).fold(0.0, f64::max) * 1.2;
    plot_ui.set_plot_bounds_y(0.0..=y_max.max(0.5));
    let stems: Vec<[f64; 2]> = visible.iter().map(|(x, y)| [*x, *y]).collect();
    plot_ui.points(Points::new("strip8", stems).stems(0.0).color(LATENCY));
});
```

## 13. Implement selection

The App holds `selection: Option<(f64, f64)>` (anchor, current). Each strip's render
function checks for the selection gestures (while stopped) and draws the band behind its
content.

### Selection state and gestures (`gui/plots.rs`)

- **State** — `App.selection: Option<(f64, f64)>` (UI state, not session data).
- **Left-drag** (from any strip, while stopped) — on drag start, `anchor = current =
  pointer x`; on drag, `current = pointer x`; on drag end, normalize to `(min, max)`. egui
  routes the continued drag to the strip where it started, so only that strip processes it.
- **Left-click** (while stopped) — clears the selection.
- **While running** — gestures inert.
- Two helpers, called by each strip: `selection_gesture(plot_ui, app)` and
  `draw_selection_band(plot_ui, app, y_min, y_max)`.

### Selection band

- Per strip, a `FilledArea` rectangle spanning `[min(anchor,current),
  max(anchor,current)] × [y_min, y_max]` (the strip's y-range), drawn behind the content —
  8 rectangles that align via the shared x-axis into one continuous band.
- The selection is used by export (point 14) and cleared on Start (point 15).

## 14. Implement the export group

The toolbar (`gui/toolbar.rs`) holds the export group (source dropdown, preview text,
Export button); the export logic lives in `gui/export.rs`. All exports are WAV, 16 kHz,
mono, 16-bit PCM, read from the history store.

### Export group UI (`gui/toolbar.rs`)

- **Source dropdown** — `ComboBox` (Raw / Post-VAD / SW chunks); the selection is stored
  in the App.
- **Preview text** — Raw → "will export selected range"; Post-VAD / SW chunks → "will
  export N chunks" (live-updated with the selection).
- **Export button** — enabled only while stopped with a selection; opens an `rfd` save
  dialog, then calls `export`.

### Export logic (`gui/export.rs`)

```rust
pub fn export(
    selection: (f64, f64),
    source: ExportSource,
    timeline: &TimelineData,
    base_path: &Path,
) -> Result<(), ExportError> { ... }
```

- **Raw** — the raw samples in `[sel_start, sel_end]`; one file at the chosen path.
- **Post-VAD** — the selected range split at gate gaps; each contiguous run of gate-passed
  audio becomes its own file (`name_0.wav`, `name_1.wav`, …).
- **SW chunks** — one file per chunk intersecting the selection (`name_0.wav`, …), full
  windows including overlap and padding.
- Chunked modes treat the chosen path as a base name and always suffix `_N.wav` (even a
  single file); Raw uses the path directly.
- WAV writing via `sepple::save_f32_to_wav` (16 kHz, mono, 16-bit PCM).
- A selection with no data in the chosen source exports zero files (preview shows N = 0).

### Preview counts

- **Raw** — always "will export selected range".
- **Post-VAD** — N = number of contiguous gate-passed runs intersecting the selection.
- **SW chunks** — N = number of chunks intersecting the selection.

## 15. Implement the GUI side of Start/Stop

`gui.rs` extends `start()`/`stop()`: Stop freezes the display, Start clears the timeline
and selection and re-attaches to the live edge.

### Start/Stop (`gui.rs`)

```rust
impl App {
    fn stop(&mut self) {
        self.running = false;
    }
    fn start(&mut self) {
        self.running = true;
        self.timeline.clear();
        self.selection = None;
    }
}
```

- **Stop** — `running = false` → the live-edge lock stops pushing (the view freezes);
  recording stops via the flag the drain checks (phase 3).
- **Start** — `running = true`, `timeline.clear()` (clears all strips/history),
  `selection = None` → the live-edge lock resumes (re-attaches to the live edge).
- The clearing is testable in phase 1 (the mockup data is cleared). The "stops recording"
  and "discards pre-Start entries" parts are no-ops until phase 3 (drain-side).
- The state indicator shows "running"/"stopped" from the `running` flag.

# Phase 2 — sepple modified for sourcing

16. Add the two additive getters to `sepple`: `VadFilter::is_talking()` for the gate
    band (strip 3) and `WordDetector::buffer()` for snapshots (strip 6).
17. Upgrade the data shapes: make `WordDetector` emit
    `DetectedWord { word, audio_span, detected_at }` with `Display` staying
    byte-identical, and give `TimestampedText` frame-derived per-symbol times produced
    inside `IpaProcessor`.

# Phase 3 — transport and full data sending

18. Build the transport from §6: one bounded channel per feed with non-blocking
    `try_send`, a shared dropped-events counter surfaced in the toolbar, and a
    per-frame GUI drain that writes into the same history store the mockup data
    populated (discarding events stamped before the Start instant).
19. Build the `instrumentation` module: `SilenceGate`, the generic `Tap<T>` used both
    as mid-pipeline processor and terminal sink, and the delegating wrappers around
    `VadFilter` / `WordDetector` that read the new accessors into their feeds.
20. Build the `pipeline` module assembling the pipeline exactly as in §3 from
    `sepple pipeline`'s configuration values (minus AudioLogger, plus SilenceGate,
    taps, wrappers, terminal tap) through `build_no_consumer`, with the terminal tap's
    sink future on a background thread and cancel-then-join on window close.
21. Replace the mockup data with the real pipeline feeds, so every strip is driven by
    live or file audio through the taps and wrappers.
22. Complete the Start/Stop path by wiring the shared signal to `SilenceGate`, so Stop
    zeroes samples for the running pipeline while the GUI freezes locally.
23. Implement the `-f` file mode behavior: the file is piped at real-time speed onto
    the session-relative timeline, with the GUI staying open and Start/Stop operable
    once the file is exhausted.
