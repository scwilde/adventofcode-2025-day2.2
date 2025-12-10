use clap::Parser;
use std::time::Instant;

/// Advent of code 2025, Day 1, Puzzle 1:
/// Counts the number of zeroes while doing a series of modulo arithmetic operations
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// Logging level to print. -v, -vv, or -vvv for levels 1, 2, or 3
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    pub logging_level: u8,

    /// Print execution time after each logged task
    #[arg(long)]
    pub benchmarking: bool,

    /// Text file containing puzzle input
    pub input_path: String
}

impl Cli {
    pub fn log_at_level<S: AsRef<str>>(&self, level: u8, msg: S) {
        if self.logging_level >= level {
            eprintln!("{}{}", "   ".repeat((level-1).into()), msg.as_ref());
        }
    }

    pub fn log_benchmark<F>(&self, level: u8, f: F)
    where
        F: FnOnce() -> ()
    {
        if self.logging_level >= level && self.benchmarking {
            let timer = Instant::now();
            f();
            eprintln!("{}Finshed in {}μs", "   ".repeat((level-1).into()), timer.elapsed().as_micros());
        }
    }
}