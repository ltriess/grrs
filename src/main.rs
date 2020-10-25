use structopt::StructOpt;
use std::io::prelude::*;
use std::io::{BufReader, Result};
use std::fs::File;
use anyhow::Context;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    for line in reader.lines().filter_map(|result| result.ok()) {
        match_line(&line, &pattern, &mut writer);
    }
}

pub fn match_line(line: &str, pattern: &str, mut writer: impl std::io::Write) {
    if line.contains(pattern) {
        writeln!(writer, "{}", line);
    }
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let f = File::open(&args.path).with_context(|| format!("could not read file {:?}", &args.path)).unwrap();
    let reader = BufReader::new(f);

    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}
