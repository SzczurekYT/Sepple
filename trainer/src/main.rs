mod audio_names_from_xml;
mod extract_audio_tar;
mod ipa_from_jsonl;

use std::{collections::HashMap, fs::File, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Converts a Wiktionary JSONL dump into an IPA dictionary
    IpaFromJsonl {
        /// Input jsonl file
        input_file: PathBuf,
        /// Output dictionary json file
        output_file: PathBuf,
    },
    /// Extracts pronunciation audio file names from a Wiktionary XML dump
    AudioNamesFromXml {
        /// Dictionary json file
        dict_file: PathBuf,
        /// Input xml file
        xml_file: PathBuf,
        /// Output csv file
        output_file: PathBuf,
        /// Language name (section header)
        lang: String,
    },
    /// Extracts audio files from a Wiktionary audio tar archive
    ExtractAudioTar {
        /// Input audio tar file
        input_file: PathBuf,
        /// Audio list csv file (word,ipa,file)
        list_file: PathBuf,
        /// Output directory for extracted audio files
        output_dir: PathBuf,
        /// Filtered output csv file (found files only)
        output_csv: PathBuf,
    },
}

fn main() {
    match Cli::parse().command {
        Command::IpaFromJsonl {
            input_file,
            output_file,
        } => {
            ipa_from_jsonl::convert_to_ipa_dictionary(
                &input_file.to_string_lossy(),
                &output_file.to_string_lossy(),
            )
            .unwrap();
        }
        Command::AudioNamesFromXml {
            dict_file,
            xml_file,
            output_file,
            lang,
        } => {
            let dictionary: HashMap<String, String> =
                serde_json::from_reader(File::open(&dict_file).unwrap()).unwrap();

            audio_names_from_xml::audio_names_from_xml(
                &xml_file.to_string_lossy(),
                &dictionary,
                &output_file.to_string_lossy(),
                &lang,
            )
            .unwrap();
        }
        Command::ExtractAudioTar {
            input_file,
            list_file,
            output_dir,
            output_csv,
        } => {
            extract_audio_tar::extract_audio_tar(
                &input_file.to_string_lossy(),
                &list_file.to_string_lossy(),
                &output_dir.to_string_lossy(),
                &output_csv.to_string_lossy(),
            )
            .unwrap();
        }
    }
}
