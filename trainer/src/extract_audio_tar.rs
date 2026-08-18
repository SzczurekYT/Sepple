use std::{
    collections::HashSet,
    fs::File,
    io,
    path::Path,
};

use indicatif::{ProgressBar, ProgressStyle};
use percent_encoding::percent_decode_str;

pub fn extract_audio_tar(
    input_file: &str,
    list_file: &str,
    output_dir: &str,
    output_csv: &str,
) -> io::Result<()> {
    let rows = read_audio_list(list_file)?;
    let wanted: HashSet<&str> = rows.iter().map(|row| row.file.as_str()).collect();
    let total = wanted.len();
    let output_dir = Path::new(output_dir);
    std::fs::create_dir_all(output_dir)?;

    let bar = ProgressBar::new(total as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} | {msg}")
            .unwrap()
            .progress_chars("##>-"),
    );

    let file = File::open(input_file)?;
    let mut archive = tar::Archive::new(file);

    let mut extracted: HashSet<String> = HashSet::new();
    let mut found: HashSet<String> = HashSet::new();

    for entry in archive.entries()? {
        let mut entry = entry?;
        let raw_path = entry.path()?.to_string_lossy().into_owned();
        let Some(rest) = raw_path.strip_prefix("audios/") else {
            continue;
        };
        if entry.header().entry_type().is_dir() {
            continue;
        }

        let clean = percent_decode_str(rest).decode_utf8_lossy().into_owned();

        if wanted.contains(clean.as_str()) && !extracted.contains(&clean) {
            extracted.insert(clean.clone());
            let out_path = output_dir.join(&clean);
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = File::create(&out_path)?;
            io::copy(&mut entry, &mut out)?;
            found.insert(clean.clone());
            bar.set_message(clean);
            bar.inc(1);
        }
    }

    bar.finish();
    println!("Done, found {}/{} audio files", found.len(), total);

    write_found_list(output_csv, &rows, &found)?;

    Ok(())
}

struct AudioRow {
    word: String,
    ipa: String,
    file: String,
}

fn read_audio_list(list_file: &str) -> io::Result<Vec<AudioRow>> {
    let mut reader = csv::Reader::from_path(list_file)?;
    let headers = reader.headers()?.clone();
    let word_idx = header_index(&headers, "word")?;
    let ipa_idx = header_index(&headers, "ipa")?;
    let file_idx = header_index(&headers, "file")?;

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record?;
        rows.push(AudioRow {
            word: record[word_idx].to_owned(),
            ipa: record[ipa_idx].to_owned(),
            file: record[file_idx].to_owned(),
        });
    }
    Ok(rows)
}

fn header_index(headers: &csv::StringRecord, name: &str) -> io::Result<usize> {
    headers
        .iter()
        .position(|h| h == name)
        .ok_or_else(|| io::Error::other(format!("csv file is missing the \"{name}\" column")))
}

fn write_found_list(
    output_csv: &str,
    rows: &[AudioRow],
    found: &HashSet<String>,
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(output_csv)?;
    writer.write_record(["word", "ipa", "file"])?;
    for row in rows {
        if found.contains(&row.file) {
            writer.write_record([&row.word, &row.ipa, &row.file])?;
        }
    }
    writer.flush()?;
    Ok(())
}
