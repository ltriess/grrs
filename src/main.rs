use structopt::StructOpt;
use std::io::prelude::*;
use std::io::BufReader;
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

fn main() {
    let args = Cli::from_args();

    let f = File::open(&args.path).with_context(|| format!("could not read file {:?}", &args.path)).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines().filter_map(|result| result.ok()) {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
