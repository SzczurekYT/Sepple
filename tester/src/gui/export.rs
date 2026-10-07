use std::ops::Range;
use std::path::{Path, PathBuf};

use sepple::{save_f32_to_wav, units::SAMPLE_RATE_U32};

use super::App;
use super::toolbar::ExportSource;

pub fn export_count(app: &App) -> Option<usize> {
    let sel = app.selection.as_ref()?;
    match app.export_source {
        ExportSource::Raw => None,
        ExportSource::PostVad => Some(intersecting_post_vad(app, sel).len()),
        ExportSource::SwChunks => Some(intersecting_chunks(app, sel).len()),
    }
}

pub fn export(app: &App, base_path: &Path) {
    let Some(sel) = app.selection.as_ref() else {
        return;
    };
    match app.export_source {
        ExportSource::Raw => {
            let samples = selected_raw_samples(app, sel);
            save_f32_to_wav(&samples, export_path(base_path, None));
        }
        ExportSource::PostVad => {
            for (i, samples) in intersecting_post_vad(app, sel).iter().enumerate() {
                save_f32_to_wav(samples, export_path(base_path, Some(i)));
            }
        }
        ExportSource::SwChunks => {
            for (i, samples) in intersecting_chunks(app, sel).iter().enumerate() {
                save_f32_to_wav(samples, export_path(base_path, Some(i)));
            }
        }
    }
}

fn intersects(start: f64, len: usize, sel: &Range<f64>) -> bool {
    let end = start + len as f64 / SAMPLE_RATE_U32 as f64;
    end >= sel.start && start <= sel.end
}

fn selected_raw_samples(app: &App, sel: &Range<f64>) -> Vec<f32> {
    let raw = &app.timeline.raw;
    let sr = SAMPLE_RATE_U32 as f64;
    let from = ((sel.start - raw.start.as_secs_f64()) * sr).round() as i64;
    let to = ((sel.end - raw.start.as_secs_f64()) * sr).round() as i64;
    let from = from.max(0) as usize;
    let to = (to as usize).min(raw.samples.len());
    if from >= to {
        return Vec::new();
    }
    raw.samples[from..to].to_vec()
}

fn intersecting_post_vad(app: &App, sel: &Range<f64>) -> Vec<Vec<f32>> {
    app.timeline
        .post_vad
        .iter()
        .filter_map(|seg| {
            if !intersects(seg.start.as_secs_f64(), seg.samples.len(), sel) {
                return None;
            }
            Some(seg.samples.clone())
        })
        .collect()
}

fn intersecting_chunks(app: &App, sel: &Range<f64>) -> Vec<Vec<f32>> {
    app.timeline
        .chunks
        .iter()
        .filter_map(|chunk| {
            if !intersects(chunk.start.as_secs_f64(), chunk.samples.len(), sel) {
                return None;
            }
            Some(chunk.samples.clone())
        })
        .collect()
}

fn export_path(base: &Path, i: Option<usize>) -> PathBuf {
    let Some(i) = i else {
        return base.with_extension("wav");
    };
    let mut name = base.file_stem().unwrap_or("exported".as_ref()).to_owned();
    name.push(format!("_{i}.wav"));
    base.with_file_name(name)
}
