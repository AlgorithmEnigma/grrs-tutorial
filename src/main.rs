use anyhow::{Context, Result};
use log::info;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug)]
struct FileReadError(String);

fn main() -> Result<()> {
    // Logging
    env_logger::init();
    info!("Searching files for occurances");

    // Handle program arguments
    let args = Cli::from_args();

    // Create stdout Buf Stream
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout);

    // Load content from file
    let content =
        File::open(&args.path).with_context(|| format!("Could not read file: {:?}", &args.path))?;
    let reader = BufReader::new(content);

    // Iterate through lines in file to search for pattern
    for line in reader.lines() {
        let line = line.with_context(|| format!("Could not read line at line"))?;
        if line.contains(&args.pattern) {
            writeln!(handle, "{:?}", line)?;
        }
    }

    info!(
        "Found all occurrences of pattern: {}, in file: {:?}",
        &args.pattern, &args.path
    );

    Ok(())
}
