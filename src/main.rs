use std::{error::Error, fs};
use clap::Parser;

use crate::string_range::StringRange;

mod cli;
mod string_range;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: cli::Cli = cli::Cli::parse();

    // cli.log_at_level(1, "Reading puzzle input...");
    // let data = fs::read_to_string(&cli.input_path);

    cli.log_at_level(1,"Testing numeric string iterator");
    cli.log_benchmark(1, || {
        let test = StringRange::new(0, 5_000_000).unwrap();
        for n in test {
            cli.log_at_level(2, format!("{}", n));
        }
    });

    cli.log_at_level(1, "Testing iterating over u64 and converting to strings");
    cli.log_benchmark(1, || {
        for n in 0..=5_000_000_u64 {
            let nstr = n.to_string();
            cli.log_at_level(2, format!("{}", nstr));
        }
    });

    Ok(())
}
