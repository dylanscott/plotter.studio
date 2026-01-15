use std::fs;
use std::io;

use clap::Parser;
use serde_json;

use plotter::simplify::{digest_svg, DigestOptions};

#[derive(Parser)]
#[command()]
struct Cli {
    path: String,

    /// Curve approximation tolerance, in inches
    #[arg(short, long)]
    tolerance: f64,

    #[arg(long, default_value_t = 96.0)]
    dpi: f32,
}

fn main() {
    let cli = Cli::parse();
    let svg_data = fs::read_to_string(cli.path).expect("Failed to read SVG file");
    let digested = digest_svg(
        &svg_data,
        &DigestOptions {
            dpi: cli.dpi,
            curve_tolerance: cli.tolerance,
        },
    )
    .expect("Failed to digest");

    serde_json::to_writer_pretty(io::stdout(), &digested).expect("Failed to convert to JSON");
}
