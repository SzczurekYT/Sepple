use std::{path::PathBuf, process::exit};

use clap::Parser;

mod gui;

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
        exit(1);
    }
}
