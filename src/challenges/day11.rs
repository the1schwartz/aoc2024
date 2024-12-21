use std::collections::HashMap;

use rayon::prelude::*;

#[path = "utils/input_reader.rs"]
mod input_reader;

fn count_digits(num: &u64) -> usize {
    if *num == 0 {
        return 1; // Special case: 0 has one digit
    }

    (*num as f64).log10().floor() as usize + 1
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let file_path = "./input/day11.txt";
    let mut total_stones = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut numbers: Vec<u64> = Vec::new();

        for line in lines.flatten() {
            numbers = line
                .split_whitespace()
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
            break;
        }

        let blinks = 25;

        for _ in 1..=blinks {
            let mut next_numbers: Vec<u64> = Vec::new();
            for n in &numbers {
                if *n == 0 {
                    next_numbers.push(1);
                } else if count_digits(n) % 2 == 0 {
                    let n_string = n.to_string();
                    let (left, right) = n_string.split_at(n_string.len() / 2);
                    let left_n = left.parse::<u64>().ok().unwrap();
                    let right_n = right.parse::<u64>().ok().unwrap();
                    next_numbers.push(left_n);
                    next_numbers.push(right_n);
                } else {
                    let n_mul_2024 = *n * 2024;
                    next_numbers.push(n_mul_2024);
                }
            }

            numbers = next_numbers;
        }

        total_stones = numbers.len();
    }
    total_stones
}

fn blink(number: u64, blinks: u32, max: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks >= max {
        return 1;
    }

    if let Some(&cache_result) = cache.get(&(number, blinks)) {
        return cache_result;
    }

    let result = match number {
        0 => blink(1, blinks + 1, max, cache),
        num if count_digits(&num) % 2 == 0 => {
            let n_string = num.to_string();
            let (left, right) = n_string.split_at(n_string.len() / 2);
            let left_n = left.parse::<u64>().ok().unwrap();
            let right_n = right.parse::<u64>().ok().unwrap();
            blink(left_n, blinks + 1, max, cache) + blink(right_n, blinks + 1, max, cache)
        }
        _ => blink(number * 2024, blinks + 1, max, cache),
    };

    cache.insert((number, blinks), result);

    result
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let file_path = "./input/day11.txt";
    let mut total_stones = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut numbers: Vec<u64> = Vec::new();

        for line in lines.flatten() {
            numbers = line
                .split_whitespace()
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
            break;
        }

        let max_blinks = 75;

        total_stones = numbers
            .par_iter()
            .map(|&v| {
                let mut local_cache: HashMap<(u64, u32), u64> = HashMap::new();
                blink(v as u64, 0, max_blinks, &mut local_cache)
            })
            .sum();
    }
    total_stones
}
