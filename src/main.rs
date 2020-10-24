use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pattern: {}, path: {:?}", self.pattern, self.path)
    }
}

fn main() {
    let args = Cli::from_args();

    println!("You called `grrs` with the following arguments: {}", args)
}
