use std::{error::Error, fs};
use clap::Parser;

use crate::string_range::StringRange;

mod cli;
mod string_range;

fn main() -> Result<(), Box<dyn Error>> {
    // let cli = cli::Cli::parse();

    // cli.log_at_level(1, "Reading puzzle input...");
    // let data = fs::read_to_string(&cli.input_path);

    let test = StringRange::new(100, 200).unwrap();
    for n in test {
        println!("{}", n);
    }


    Ok(())
}
