#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern : String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    hello_rust::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())

}


fn answer() -> i8 {
    return 42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}