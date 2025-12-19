use std::error::Error;
use std::fs;

use crate::cli;
use crate::utils;

pub fn solve(cli: &cli::Cli, is_id_valid: fn(u64) -> bool) -> Result<(u64, u64), Box<dyn Error>> {
    cli.log_at_level(1, "Reading puzzle input...");
    let data = fs::read_to_string(&cli.input_path)?;

    cli.log_at_level(1, "Parsing ID Ranges...");
    let mut parsed_range_count = 0;
    let mut total_parsed_id_count = 0;
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
        let mut range_parsed_id_count = 0;
        for id in id_range.lower..=id_range.upper {
            total_parsed_id_count += 1;
            range_parsed_id_count += 1;
            match is_id_valid(id) {
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
            "Found {} invalid IDs out of {} total. New invalid ID sum {}",
            range_invalid_id_count, range_parsed_id_count, invalid_id_sum
        ));
        parsed_range_count += 1;
    }

    cli.log_at_level(1, format!(
        "Parsed {} ID ranges and found {} invalid IDs out of {} total. Invalid ID sum:",
        parsed_range_count, total_invalid_id_count, total_parsed_id_count
    ));
    println!("{}", invalid_id_sum);

    Ok((total_invalid_id_count, invalid_id_sum))
}

