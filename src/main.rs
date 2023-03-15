mod block;
mod game;
mod play;
mod ai;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    Normal,
    Auto,
}

fn main() {
    let cli = Cli::parse();
    match cli.mode {
        None |
        Some(Mode::Normal) => {
            play::normal();
        }
        Some(Mode::Auto) => {
            play::auto();
        }
    }
}
