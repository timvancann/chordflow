use std::path::PathBuf;

use clap::*;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(
        long,
        value_name = "INT",
        help = "BPM (Beats per minute)",
        default_value_t = 100
    )]
    pub bpm: usize,

    #[arg(
        short,
        long,
        value_name = "INT",
        help = "Number of bars per chord",
        default_value_t = 2
    )]
    pub bars_per_chord: usize,

    #[arg(
        short,
        long,
        value_name = "INT",
        help = "Number of beats per bar",
        default_value_t = 4
    )]
    pub ticks_per_bar: usize,

    #[arg(short, long, help = "Soundfont file path")]
    pub soundfont: Option<PathBuf>,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
