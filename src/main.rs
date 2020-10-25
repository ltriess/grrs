use structopt::StructOpt;
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

fn main() -> Result<()> {
    let args = Cli::from_args();

    let f = File::open(&args.path).with_context(|| format!("could not read file {:?}", &args.path)).unwrap();
    let reader = BufReader::new(f);

    grrs::find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}
