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
    let transport = todo!();
    if let Err(err) = gui::run(transport) {
        eprintln!("tester failed: {err:?}");
        exit(1);
    }
}
