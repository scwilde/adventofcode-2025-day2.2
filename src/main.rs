use std::{error::Error, fs};
use clap::Parser;

use crate::string_range::StringRange;

mod cli;
mod string_range;

fn main() -> Result<(), Box<dyn Error>> {
    // let cli = cli::Cli::parse();

    // cli.log_at_level(1, "Reading puzzle input...");
    // let data = fs::read_to_string(&cli.input_path);

    let test = StringRange::new(0, 5_000_000).unwrap();
    for n in test {
        println!("{}", n.trim_start_matches("0"));
    }

    for n in 0..=5_000_000_u64 {
        let nstr = n.to_string();
        println!("{}", n)
    }

    Ok(())
}
