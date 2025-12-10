use std::error::Error;

pub struct Range {
    pub lower: u64,
    pub upper: u64,
}

impl Range {
    pub fn from_string(range_string: &str) -> Result<Self, Box<dyn Error>>{
        let mut splitrange = range_string.split("-");
        let lower = splitrange.next()
            .ok_or(format!("{} not a valid range.", range_string))?
            .parse().map_err(|err| format!("Problem parsing range {}: {}", range_string, err))?;
        let upper = splitrange.next()
            .ok_or(format!("{} not a valid range.", range_string))?
            .parse().map_err(|err| format!("Problem parsing range {}: {}", range_string, err))?;
        match splitrange.next() {
            Some(_) => Err(format!("{} not a valid range.", range_string).into()),
            None => Ok(Range{lower: lower, upper: upper})
        }
    }
}