use std::collections::VecDeque;
use itertools::Itertools;

pub fn is_id_valid_my(id: u64) -> bool {
    if id < 11 { return true }

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

pub fn is_id_valid_vec(id: u64) -> bool {
    if id < 11 { return true }

    // let id: Vec<_> = id.to_string().chars().collect();
    // let base: Vec<_> = id.iter().collect();
    // let mut test: Vec<_> = id.iter().collect();
    let base: Vec<_> = id.to_string().chars().collect();
    let mut test = base.clone();

    for _ in 1..=base.len()/2 {
        test.rotate_right(1);
        if base == test {return false;}
    }

    true
}

pub fn is_id_valid_vecdeque(id: u64) -> bool {
    if id < 11 { return true }

    // let id: Vec<_> = id.to_string().chars().collect();
    // let base: VecDeque<_> = id.iter().collect();
    // let mut test: VecDeque<_> = id.iter().collect();
    let base: VecDeque<_> = id.to_string().chars().collect();
    let mut test = base.clone();

    for _ in 1..=base.len()/2 {
        test.rotate_right(1);
        if base == test { return false }
    }

    true
}

pub fn is_id_valid_kyuu(id: u64) -> bool {
    if id < 11 { return true }

    let id = id.to_string();
    for i in 1..=id.len()/2 {
        if id[i..] == id[..id.len()-i] { return false }
    }

    true
}

pub fn is_id_valid_numeric(id: u64) -> bool {
    // Find number of digits and highest digit unit.
    // This computes id.ilog10() and 10u64.pow(id.ilog10()) simultaneously.
    let mut highest_digit_unit = 1;
    let mut total_digits = 1;
    if highest_digit_unit * const { 10u64.pow(10) } <= id {
        highest_digit_unit *= const { 10u64.pow(10) };
        total_digits += 10;
    }
    if highest_digit_unit * const { 10u64.pow(5) } <= id {
        highest_digit_unit *= const { 10u64.pow(5) };
        total_digits += 5;
    }
    while highest_digit_unit * 10 <= id {
        highest_digit_unit *= 10;
        total_digits += 1;
    }
    
    // Rotate digits and check for equality.
    let mut candidate = id;
    for _ in 0..total_digits/2 {
        let digit = candidate % 10;
        candidate /= 10;
        candidate += digit * highest_digit_unit;
        if candidate == id { return false; }
    }

    true
}