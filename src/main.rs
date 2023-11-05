mod output;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Args {
    /// Pattern to look
    #[arg()]
    pattern: String,
    /// Path of the file to read
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> Result<()> {
    let Args { path, pattern } = Args::parse();

    let buffer = json2ts::get_buffer(&path)
        .with_context(|| format!("Error reading `{}`", path.to_str().unwrap()))?;

    let matches = json2ts::get_matches(buffer, &pattern);

    let mut output_channel = output::get_output_channel();

    for pattern_match in matches {
        output_channel.write(&pattern_match);
    }

    Ok(())
}
