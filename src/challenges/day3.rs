#[path = "utils/input_reader.rs"]
mod input_reader;

use regex::Regex;

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file_path = "./input/day3.txt";
    let mut multiplication_sum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

        for line in lines.flatten() {
            let mut current = line.clone();

            loop {
                let do_split: Vec<&str> = current.splitn(2, "do()").collect();
                let do_split_first = *do_split.get(0).unwrap();

                for m in mul_regex.find_iter(do_split_first) {
                    let match_string = m.as_str();

                    let cleaned_match = match_string.replace("mul(", "").replace(")", "");
                    let parts: Vec<&str> = cleaned_match.split(",").collect();
                    let left: i32 = parts.get(0).unwrap().parse().unwrap();
                    let right: i32 = parts.get(1).unwrap().parse().unwrap();

                    multiplication_sum += left * right;
                }

                if do_split.len() == 1 {
                    break;
                }
                current = String::from(*do_split.get(1).unwrap());
            }
        }
    }

    multiplication_sum
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file_path = "./input/day3.txt";
    let mut multiplication_sum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

        let mut multiplication_enabled = true;

        for line in lines.flatten() {
            let mut current = line.clone();

            loop {
                let do_split: Vec<&str> = current.splitn(2, "do()").collect();
                let do_split_first = *do_split.get(0).unwrap();

                let dont_split: Vec<&str> = do_split_first.splitn(2, "don't()").collect();
                let do_part = *dont_split.get(0).unwrap();

                for m in mul_regex.find_iter(do_part) {
                    let match_string = m.as_str();

                    let cleaned_match = match_string.replace("mul(", "").replace(")", "");
                    let parts: Vec<&str> = cleaned_match.split(",").collect();
                    let left: i32 = parts.get(0).unwrap().parse().unwrap();
                    let right: i32 = parts.get(1).unwrap().parse().unwrap();

                    if multiplication_enabled {
                        multiplication_sum += left * right;
                    }
                }

                if dont_split.len() == 2 {
                    multiplication_enabled = false;
                }

                if do_split.len() == 1 {
                    break;
                }

                multiplication_enabled = true;
                current = String::from(*do_split.get(1).unwrap());
            }
        }
    }

    multiplication_sum
}
