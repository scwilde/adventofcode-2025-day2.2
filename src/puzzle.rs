use std::error::Error;
use std::fs;

use crate::cli;
use crate::utils;

pub struct Data {
    parsed_range_count: i32,
    parsed_id_count: i32,
    ids: Vec<u64>
}

pub fn setup(cli: &cli::Cli) -> Result<Data, Box<dyn Error>> {
    cli.log_at_level(2, "Reading puzzle input...");
    let input = fs::read_to_string(cli.input_path.clone())?;

    cli.log_at_level(2, "Parsing ID Ranges...");
    let mut data = Data{
        parsed_range_count: 0,
        parsed_id_count: 0,
        ids: Vec::new()
    };
    for id_range in input.split(",") {
        cli.log_at_level(3, format!("Parsing {}...", id_range));
        let id_range = utils::Range::from_string(id_range)?;
        let mut range_id_count = 0;
        for id in id_range.lower..=id_range.upper {
            data.ids.push(id);
            data.parsed_id_count += 1;
            range_id_count += 1;
            cli.log_at_level(4, format!("Queing {}...", id));
        }
        cli.log_at_level(3, format!(
            "Queued {} IDs from range {}..={}",
            range_id_count, id_range.lower, id_range.upper
        ));
        data.parsed_range_count += 1;
    }

    Ok(data)
}

pub fn solve(cli: &cli::Cli, data: &Data, is_id_valid: fn(u64) -> bool) -> (u64, u64) {
    let mut invalid_id_count = 0;
    let mut invalid_id_sum = 0;
    
    for id in data.ids.iter() {
        match is_id_valid(*id) {
            false => {
                cli.log_at_level(2, format!("{} is not a valid ID!", id));
                invalid_id_count += 1;
                invalid_id_sum += id;
            }
            true => cli.log_at_level(2, format!("{} is a valid ID!", id))
        }
    }

    (invalid_id_count, invalid_id_sum)
}

