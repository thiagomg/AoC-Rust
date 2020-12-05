use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run() -> Result<(), std::io::Error> {
    let numbers: Vec<u32> = read_file()?;

    if let Some((n1, n2)) = find_brute_force(&numbers, 2020) {
        println!("Result = {} * {} = {}", n1, n2, n1*n2);
    }

    if let Some((n1, n2)) = find_optimized(&numbers, 2020) {
        println!("Result = {} * {} = {}", n1, n2, n1*n2);
    }

    if let Some((n1, n2, n3)) = find_brute_force_3(&numbers, 2020) {
        println!("Result = {} * {} * {} = {}", n1, n2, n3, n1*n2*n3);
    }

    Ok(())
}

fn find_optimized(numbers: &Vec<u32>, sum: u32) -> Option<(u32, u32)> {
    let nums : HashSet<u32> = HashSet::from_iter(numbers.iter().cloned());
    for number in numbers {
        let diff = sum - *number;
        // FIXME: This is not technically correct, as sum/2 could return itself and not necessarily a duplicate
        if nums.contains(&diff) {
            return Some((*number, diff));
        }
    }
    None
}

fn find_brute_force(numbers: &Vec<u32>, sum: u32) -> Option<(u32, u32)> {
    // Brute force
    for (pos, number) in numbers.iter().enumerate() {
        for (other_pos, other_number) in numbers.iter().enumerate() {
            if pos == other_pos {
                continue;
            }
            if number + other_number == sum {
                return Some((*number, *other_number));
            }
        }
    }
    None
}

fn find_brute_force_3(numbers: &Vec<u32>, sum: u32) -> Option<(u32, u32, u32)> {
    // Brute force
    for (first_pos, first_num) in numbers.iter().enumerate() {
        for (second_pos, second_num) in numbers.iter().enumerate() {
            for (third_pos, third_num) in numbers.iter().enumerate() {
                if first_pos == second_pos || first_pos == third_pos || second_pos == third_pos {
                    continue;
                }
                if first_num + second_num + third_num == sum {
                    return Some((*first_num, *second_num, *third_num));
                }
            }
        }
    }
    None
}

// FIXME: I could collect straight to a hash set. Need to figure out generics
fn read_file() -> Result<Vec<u32>, std::io::Error> {
    let file = File::open("data/day1/input")?;
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(lines)
}
