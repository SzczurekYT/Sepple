use std::{path::PathBuf, process::exit};

use clap::Parser;

pub(crate) mod gui;
mod instrumentation;
mod pipeline;
mod transport;

#[derive(Parser)]
#[command(version, about = "live pipeline debugger", long_about = None)]
struct Cli {
    /// Feed a WAV file instead of the live microphone
    #[arg(short = 'f', long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let transport = match pipeline::build(cli.file) {
        Ok(transport) => transport,
        Err(err) => {
            eprintln!("tester failed: {err:?}");
            exit(1);
        }
    };
    if let Err(err) = gui::run(transport) {
        eprintln!("tester failed: {err:?}");
        exit(1);
    }
}
