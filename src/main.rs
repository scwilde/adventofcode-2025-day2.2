use std::error::Error;
use std::time::Instant;
use clap::Parser;

mod cli;
mod puzzle;
mod utils;
mod id;

fn benchmark<S: AsRef<str>>(cli: &cli::Cli, msg: S, solve_method: fn(u64) -> bool, data: &puzzle::Data) -> Result<(), Box<dyn Error>> {
    println!("{}", msg.as_ref());
    
    let timer = Instant::now();
    match (cli.expected_invalid_id_ct, cli.expected_invalid_id_sum) == puzzle::solve(cli, data, solve_method) {
        true => { println!("Puzzle solved correctly in {}s without black box", timer.elapsed().as_secs_f32()) }
        false => { println!("Puzzle solved incorrectly in {}s without black box", timer.elapsed().as_secs_f32()) }
    }

    let timer = Instant::now();
    let solution = std::hint::black_box((cli.expected_invalid_id_ct, cli.expected_invalid_id_sum));
    let answer = std::hint::black_box(puzzle::solve(cli, data, solve_method));
    match solution == answer {
        true => { println!("Puzzle solved correctly in {}s with black box", timer.elapsed().as_secs_f32()) }
        false => { println!("Puzzle solved incorrectly in {}s with black box", timer.elapsed().as_secs_f32()) }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();
    let data = puzzle::setup(&cli)?;

    benchmark(
        &cli,
        "Benchmarking my (bad) solution...",
        id::is_id_valid_my,
        &data
    )?;
    benchmark(
        &cli,
        "Benchmarking regular Vec rotation...",
        id::is_id_valid_vec,
        &data
    )?;
    benchmark(
        &cli,
        "Benchmarking VecDeque rotation...",
        id::is_id_valid_vecdeque,
        &data
    )?;
    benchmark(
        &cli,
        "Benchmarking kyuuhachi's string slice solution...",
        id::is_id_valid_kyuu,
        &data
    )?;
    benchmark(
        &cli,
        "Benchmarking orlp's numeric solution...",
        id::is_id_valid_orlp,
        &data
    )?;

    Ok(())
}
