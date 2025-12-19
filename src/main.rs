use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use clap::Parser;
use itertools::Itertools;

mod cli;
mod utils;

fn is_id_valid(id: u64) -> bool {
    let id_string = id.to_string();
    let id_len = id_string.len();
    (1..id_len)
        .filter(|divisor| id_len % divisor == 0)
        .all(|divisor| {
            let chunks = id_string.chars().chunks(divisor);
            let base = chunks.into_iter().next().unwrap().collect::<String>();
            chunks.into_iter().any(|chunk| chunk.collect::<String>() != base)
        })
}

fn vd_is_id_valid(id: u64) -> bool {
    let id: Vec<_> = id.to_string().chars().collect();
    let base: VecDeque<_> = id.iter().collect();
    let mut test: VecDeque<_> = id.iter().collect();
    test.rotate_right(1);

    for _ in 1..=id.len()/2 {
        if base == test {return false;}
        test.rotate_right(1);
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: cli::Cli = cli::Cli::parse();

    cli.log_at_level(1, "Reading puzzle input...");
    let data = fs::read_to_string(&cli.input_path)?;

    cli.log_at_level(1, "Parsing ID Ranges...");
    let mut parsed_range_count = 0;
    let mut total_invalid_id_count = 0;
    let mut invalid_id_sum = 0;
    for id_range in data.split(",") {
        cli.log_at_level(2, format!("Parsing {}...", id_range));
        let id_range = utils::Range::from_string(id_range)?;

        cli.log_at_level(2, format!(
            "Searching range {}..={} for invalid IDs...",
            id_range.lower, id_range.upper
        ));
        let mut range_invalid_id_count = 0;
        for id in id_range.lower..=id_range.upper {
            match vd_is_id_valid(id) {
                false => {
                    cli.log_at_level(3, format!("{} is not a valid ID!", id));
                    range_invalid_id_count += 1;
                    total_invalid_id_count += 1;
                    invalid_id_sum += id;
                }
                true => cli.log_at_level(3, format!("{} is a valid ID!", id)),
            }
        }
        cli.log_at_level(2, format!(
            "Found {} invalid IDs. New invalid ID sum {}",
            range_invalid_id_count, invalid_id_sum
        ));
        parsed_range_count += 1;
    }

    cli.log_at_level(1, format!(
        "Parsed {} ID ranges and found {} invalid IDs. Invalid ID sum:",
        parsed_range_count, total_invalid_id_count
    ));
    println!("{}", invalid_id_sum);

    Ok(())
}
