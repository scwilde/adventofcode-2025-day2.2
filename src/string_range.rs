enum Status {
    NotStarted,
    Started,
    Finished
}

pub struct StringRange {
    status: Status,
    current: String,
    end: String
}

fn increment_digit(numeric_string: &mut String, n: usize) {
    let c = numeric_string.chars().nth(n);
    match c {
        Some('0') => numeric_string.replace_range(n..=n, "1"),
        Some('1') => numeric_string.replace_range(n..=n, "2"),
        Some('2') => numeric_string.replace_range(n..=n, "3"),
        Some('3') => numeric_string.replace_range(n..=n, "4"),
        Some('4') => numeric_string.replace_range(n..=n, "5"),
        Some('5') => numeric_string.replace_range(n..=n, "6"),
        Some('6') => numeric_string.replace_range(n..=n, "7"),
        Some('7') => numeric_string.replace_range(n..=n, "8"),
        Some('8') => numeric_string.replace_range(n..=n, "9"),
        Some('9') => numeric_string.replace_range(n..=n, "0"),
        Some(_) => panic!("Invalid character '{}'", c.unwrap()),
        None => panic!("No character at index {}", n)
    };
}

impl StringRange {
    pub fn new(start: u64, end: u64) -> Result<Self, String> {
        // TODO: pad out both numbers to be the same length
        if start > end {
            return Err(format!("Start value ({}) occurs after end ({}) in sequence", start, end));
        }

        Ok(StringRange{
            status: Status::NotStarted,
            current: format!("{}", start),
            end: format!("{}", end)
        })
    }
}

impl Iterator for StringRange {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.status {
            Status::NotStarted => {
                self.status = Status::Started;
                Some(self.current.clone())
            }
            // TODO change from using newval to instead iterating over a clone of self.current
            // TODO then we can make increment_digit directly increment the self.current
            Status::Started => {
                let mut cascading = false;
                let mut newval = self.current.clone();
                for (n, c) in self.current.char_indices().rev() {
                    match (cascading, c) {
                        (false, '9') => { increment_digit(&mut newval, n);cascading = true; }
                        (true, '9') => { increment_digit(&mut newval, n); }
                        _ => { increment_digit(&mut newval, n); break; }
                    }
                }
                
                self.current = newval.clone();
                if self.current == self.end {
                    self.status = Status::Finished
                }

                Some(newval)
            }
            Status::Finished => None
        }
    }
}