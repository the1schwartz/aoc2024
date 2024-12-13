use std::{fs::File, io::BufReader};

#[path = "utils/input_reader.rs"]
mod input_reader;

fn load_data(lines: std::io::Lines<BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in lines.flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let left = parts.get(0).unwrap();
        let right = parts.get(1).unwrap();
        let left_value: i32 = left.parse().unwrap();
        let right_value: i32 = right.parse().unwrap();
        left_list.push(left_value);
        right_list.push(right_value);
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file_path = "./input/day1.txt";
    let mut total_distance = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let data = load_data(lines);
        let left_list = data.0;
        let right_list = data.1;

        for i in 0..left_list.len() {
            let left = left_list.get(i).unwrap();
            let right = right_list.get(i).unwrap();
            let distance: i32 = left - right;
            total_distance += distance.abs();
        }
    }

    total_distance
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file_path = "./input/day1.txt";

    let mut similarity_score = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let data = load_data(lines);
        let left_list = data.0;
        let right_list = data.1;

        for l in &left_list {
            let mut count = 0;
            for r in &right_list {
                if l == r {
                    count += 1;
                }
            }
            let score = l * count;
            similarity_score += score;
        }
    }

    similarity_score
}
