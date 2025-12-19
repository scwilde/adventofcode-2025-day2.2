use std::error::Error;
use std::time::Instant;
use clap::Parser;

mod cli;
mod puzzle;
mod utils;
mod id;

fn benchmark<S: AsRef<str>>(msg: S, solve_method: fn(u64) -> bool, cli: &cli::Cli) -> Result<(), Box<dyn Error>> {
    println!("{}", msg.as_ref());
    
    let timer = Instant::now();
    match (cli.expected_invalid_id_ct, cli.expected_invalid_id_sum) == puzzle::solve(cli, solve_method)? {
        true => { println!("Puzzle solved correctly in {}s without black box", timer.elapsed().as_secs_f32()) }
        false => { println!("Puzzle solved incorrectly in {}s without black box", timer.elapsed().as_secs_f32()) }
    }

    let timer = Instant::now();
    let solution = std::hint::black_box((cli.expected_invalid_id_ct, cli.expected_invalid_id_sum));
    let answer = std::hint::black_box(puzzle::solve(cli, solve_method)?);
    match solution == answer {
        true => { println!("Puzzle solved correctly in {}s with black box", timer.elapsed().as_secs_f32()) }
        false => { println!("Puzzle solved incorrectly in {}s with black box", timer.elapsed().as_secs_f32()) }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();

    benchmark(
        "Benchmarking my (bad) solution...",
        id::is_id_valid_my,
        &cli
    )?;
    benchmark(
        "Benchmarking regular Vec rotation...",
        id::is_id_valid_vec,
        &cli
    )?;
    benchmark(
        "Benchmarking VecDeque rotation...",
        id::is_id_valid_vecdeque,
        &cli
    )?;
    benchmark(
        "Benchmarking kyuuhachi's string slice solution...",
        id::is_id_valid_kyuu,
        &cli
    )?;
    benchmark(
        "Benchmarking orlp's numeric solution...",
        id::is_id_valid_orlp,
        &cli
    )?;

    Ok(())
}
