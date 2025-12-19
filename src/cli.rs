use clap::Parser;

/// Advent of code 2025, Day 1, Puzzle 1:
/// Counts the number of zeroes while doing a series of modulo arithmetic operations
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Logging level to print. -v, -vv, or -vvv for levels 1, 2, or 3
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub logging_level: u8,
    
    /// Text file containing puzzle input
    pub input_path: String,

    /// How many invalid IDs there should be if the puzzle is solved correctly
    pub expected_invalid_id_ct: u64,

    /// Sum of all the invalid IDs when the puzzle is solved correctly
    pub expected_invalid_id_sum: u64,
}

impl Cli {
    pub fn log_at_level<S: AsRef<str>>(&self, level: u8, msg: S) {
        if self.logging_level >= level {
            eprintln!("{}{}", "   ".repeat((level-1).into()), msg.as_ref());
        }
    }

    // pub fn log_benchmark<F, T>(&self, level: u8, f: F) -> T
    // where
    //     F: FnOnce() -> T
    // {
    //     if self.logging_level >= level && self.benchmarking {
    //         let timer = Instant::now();
    //         f();
    //         eprintln!("{}Finshed in {}μs", "   ".repeat((level-1).into()), timer.elapsed().as_micros());
    //     }
    // }
}