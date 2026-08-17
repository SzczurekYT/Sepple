use std::{collections::HashSet, fs::File, io, path::Path};

use indicatif::{ProgressBar, ProgressStyle};
use percent_encoding::percent_decode_str;

pub fn extract_audio_tar(
    input_file: &str,
    list_file: &str,
    output_dir: &str,
    output_csv: &str,
) -> io::Result<()> {
    let rows = read_audio_list(list_file)?;
    let wanted: HashSet<String> = rows.iter().map(|row| decode_file_name(&row.file)).collect();
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

    for entry in archive.entries()? {
        let mut entry = entry?;
        let raw_path = entry.path()?.to_string_lossy().into_owned();
        let Some(rest) = raw_path.strip_prefix("audios/") else {
            continue;
        };
        if entry.header().entry_type().is_dir() {
            continue;
        }

        let clean = decode_file_name(rest);

        if let Some(key) = find_entry_for_tar_name(&clean, &wanted)
            && !extracted.contains(&key)
        {
            extracted.insert(key.clone());
            let out_path = output_dir.join(&key);
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = File::create(&out_path)?;
            io::copy(&mut entry, &mut out)?;
            bar.set_message(key);
            bar.inc(1);
        }
    }

    bar.finish();

    let words_satisfied = rows
        .iter()
        .filter(|row| extracted.contains(&decode_file_name(&row.file)))
        .count();
    println!(
        "Done, extracted {} files for {}/{} words",
        extracted.len(),
        words_satisfied,
        rows.len()
    );

    write_found_list(output_csv, &rows, &extracted)?;

    Ok(())
}

/// Converts a name to its human readable
/// form from what MediaWiki uses internally
fn decode_file_name(name: &str) -> String {
    percent_decode_str(name)
        .decode_utf8_lossy()
        .replace('_', " ")
}

/// Finds a matching entry in the list for a file name from tar
/// accomodating for things like different extension suffixes due
/// to transcoding
fn find_entry_for_tar_name(tar_file_name: &str, wanted: &HashSet<String>) -> Option<String> {
    if wanted.contains(tar_file_name) {
        return Some(tar_file_name.to_owned());
    }
    for ext in [".ogg", ".mp3", ".oga"] {
        if let Some(base) = tar_file_name.strip_suffix(ext) {
            let stem = base.rsplit('/').next().unwrap_or(base);
            if stem.contains('.') && wanted.contains(base) {
                return Some(base.to_owned());
            }
        }
    }
    None
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
        let name = decode_file_name(&row.file);
        if found.contains(&name) {
            writer.write_record([&row.word, &row.ipa, &name])?;
        }
    }
    writer.flush()?;
    Ok(())
}
