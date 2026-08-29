use std::{io, path::Path, process::Command, time::Instant};

use hound::{SampleFormat, WavSpec};
use sepple::{SeppleBackend, ipa_recognizer::IpaRecognizer};

const BATCH_SIZE: usize = 10;

pub fn add_inferred_ipa(
    input_file: &str,
    output_csv: &str,
    audio_dir: &str,
    model_path: &str,
) -> io::Result<()> {
    let mut rows = read_rows(input_file)?;

    println!("Loading model");
    let load_start = Instant::now();
    let recognizer = IpaRecognizer::<SeppleBackend>::init_default(model_path);
    println!("Load done (took: {:.2?})", load_start.elapsed());

    let mut generated = 0u64;
    let mut failed = 0u32;
    let mut skipped = 0u32;
    for i in 0..rows.len() {
        if rows[i].is_complete() {
            skipped += 1;
            continue;
        }
        match generate_row(&mut rows[i], audio_dir, &recognizer) {
            Ok(()) => generated += 1,
            Err(e) => {
                failed += 1;
                eprintln!("skipping '{}': {e}", rows[i].file);
            }
        }
        if generated > 0 && generated as usize % BATCH_SIZE == 0 {
            write_all(output_csv, &rows)?;
            println!(
                "checkpoint: wrote {} rows after {generated} generated ({skipped} skipped, {failed} failed)",
                rows.len()
            );
        }
    }

    write_all(output_csv, &rows)?;
    println!(
        "Done, generated {generated} rows ({skipped} already done, {failed} failed, {} total)",
        rows.len()
    );
    Ok(())
}

fn generate_row(
    row: &mut Row,
    audio_dir: &str,
    recognizer: &IpaRecognizer<SeppleBackend>,
) -> io::Result<()> {
    let path = Path::new(audio_dir).join(&row.file);
    let samples = read_audio_to_f32(&path)?;
    let inferred = recognizer.recognize(&samples);
    let distance = 1.0 - phonetics::confusion::similarity(&inferred, &row.ipa);
    row.inferred_ipa = Some(inferred);
    row.ipa_distance = Some(format!("{distance:.6}"));
    Ok(())
}

fn read_audio_to_f32(path: &Path) -> io::Result<Vec<f32>> {
    // ffmpeg writes invalid sizes (0xFFFFFFFF) when muxing to a pipe
    // (it can't seek back on a non-regular file), so transcode to a
    // temp wav file and let it patch the header properly.
    let tmp = std::env::temp_dir().join(format!("sepple_ipa_{}.wav", std::process::id()));
    let status = Command::new("ffmpeg")
        .args([
            "-v",
            "error",
            "-y",
            "-i",
            &path.to_string_lossy(),
            "-ar",
            "16000",
            "-ac",
            "1",
            "-acodec",
            "pcm_s16le",
            "-f",
            "wav",
            &tmp.to_string_lossy(),
        ])
        .status()?;

    if !status.success() {
        let _ = std::fs::remove_file(&tmp);
        return Err(io::Error::other(format!("ffmpeg failed with {status}")));
    }

    let samples = {
        let mut reader =
            hound::WavReader::open(&tmp).map_err(|e| io::Error::other(e.to_string()))?;
        let spec: WavSpec = reader.spec();
        if spec.channels != 1
            || spec.sample_rate != 16000
            || spec.bits_per_sample != 16
            || spec.sample_format != SampleFormat::Int
        {
            return Err(io::Error::other(format!(
                "unexpected wav spec {spec:?} from ffmpeg"
            )));
        }

        reader
            .samples::<i16>()
            .map(|s| s.unwrap() as f32 / (i16::MAX as f32 + 1.0))
            .collect::<Vec<f32>>()
    };
    std::fs::remove_file(&tmp)?;

    if samples.is_empty() {
        return Err(io::Error::other("decoded audio is empty"));
    }
    Ok(samples)
}

struct Row {
    word: String,
    ipa: String,
    file: String,
    inferred_ipa: Option<String>,
    ipa_distance: Option<String>,
}

impl Row {
    fn is_complete(&self) -> bool {
        self.inferred_ipa
            .as_deref()
            .map(str::trim)
            .is_some_and(|s| !s.is_empty())
            && self
                .ipa_distance
                .as_deref()
                .map(str::trim)
                .is_some_and(|s| !s.is_empty())
    }
}

fn read_rows(input_file: &str) -> io::Result<Vec<Row>> {
    let mut reader = csv::Reader::from_path(input_file)?;
    let headers = reader.headers()?.clone();
    let word_idx = header_index(&headers, "word")?;
    let ipa_idx = header_index(&headers, "ipa")?;
    let file_idx = header_index(&headers, "file")?;
    let inferred_ipa_idx = header_index_opt(&headers, "inferred_ipa");
    let ipa_distance_idx = header_index_opt(&headers, "ipa_distance");

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record?;
        rows.push(Row {
            word: record[word_idx].to_owned(),
            ipa: record[ipa_idx].to_owned(),
            file: record[file_idx].to_owned(),
            inferred_ipa: inferred_ipa_idx.map(|i| record[i].to_owned()),
            ipa_distance: ipa_distance_idx.map(|i| record[i].to_owned()),
        });
    }
    Ok(rows)
}

fn write_all(output_csv: &str, rows: &[Row]) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(output_csv)?;
    writer.write_record(["word", "ipa", "file", "inferred_ipa", "ipa_distance"])?;
    for row in rows {
        writer.write_record([
            &row.word,
            &row.ipa,
            &row.file,
            row.inferred_ipa.as_deref().unwrap_or(""),
            row.ipa_distance.as_deref().unwrap_or(""),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn header_index(headers: &csv::StringRecord, name: &str) -> io::Result<usize> {
    header_index_opt(headers, name)
        .ok_or_else(|| io::Error::other(format!("csv file is missing the \"{name}\" column")))
}

fn header_index_opt(headers: &csv::StringRecord, name: &str) -> Option<usize> {
    headers.iter().position(|h| h == name)
}
