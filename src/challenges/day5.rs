#[path = "utils/input_reader.rs"]
mod input_reader;

use std::collections::HashMap;

fn get_index(vector: &Vec<i32>, value: i32) -> Option<usize> {
    if let Some(index) = vector.iter().position(|&x| x == value) {
        return Some(index);
    }

    return None;
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file_path = "./input/day5.txt";
    let mut middle_number_sum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut before_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut after_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut add_rules = true;

        for line in lines.flatten() {
            if line.is_empty() {
                add_rules = false;
                continue;
            }

            if add_rules {
                let rule: Vec<i32> = line
                    .split("|")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                let left = rule[0];
                let right = rule[1];
                before_rules
                    .entry(left)
                    .or_insert_with(Vec::new)
                    .push(right);
                after_rules.entry(right).or_insert_with(Vec::new).push(left);
            } else {
                let update: Vec<i32> = line
                    .split(",")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();

                let mut correct_order = true;
                for u in &update {
                    let index = get_index(&update, *u).unwrap();

                    if let Some(before) = before_rules.get(&u) {
                        for b in before {
                            if let Some(i) = get_index(&update, *b) {
                                if index > i {
                                    correct_order = false;
                                    break;
                                }
                            }
                        }
                    }

                    if correct_order {
                        if let Some(after) = after_rules.get(&u) {
                            for a in after {
                                if let Some(i) = get_index(&update, *a) {
                                    if index < i {
                                        correct_order = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if correct_order {
                    if let Some(middle) = update.get(update.len() / 2) {
                        middle_number_sum += middle;
                    }
                }
            }
        }
    }

    middle_number_sum
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file_path = "./input/day5.txt";
    let mut middle_number_sum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut before_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut after_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut add_rules = true;

        for line in lines.flatten() {
            if line.is_empty() {
                add_rules = false;
                continue;
            }

            if add_rules {
                let rule: Vec<i32> = line
                    .split("|")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                let left = rule[0];
                let right = rule[1];
                before_rules
                    .entry(left)
                    .or_insert_with(Vec::new)
                    .push(right);
                after_rules.entry(right).or_insert_with(Vec::new).push(left);
            } else {
                let mut update: Vec<i32> = line
                    .split(",")
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();

                let update_copy = update.clone();
                let mut incorrect_order = false;

                for u in &update_copy {
                    let mut correct_order = true;
                    let index = get_index(&update, *u).unwrap();

                    if let Some(before) = before_rules.get(&u) {
                        let mut lowest_index = usize::max_value();
                        for b in before {
                            if let Some(i) = get_index(&update, *b) {
                                if index > i {
                                    correct_order = false;
                                    lowest_index = std::cmp::min(lowest_index, i);
                                }
                            }
                        }

                        if !correct_order {
                            update.remove(index);
                            update.insert(lowest_index, *u);
                            incorrect_order = true;
                        }
                    }
                    correct_order = true;

                    if let Some(after) = after_rules.get(&u) {
                        let mut highest_index: usize = 0;
                        for a in after {
                            if let Some(i) = get_index(&update, *a) {
                                if index < i {
                                    correct_order = false;
                                    highest_index = std::cmp::max(highest_index, i);
                                }
                            }
                        }

                        if !correct_order {
                            update.remove(index);
                            update.insert(highest_index, *u);
                            incorrect_order = true;
                        }
                    }
                }

                if incorrect_order {
                    if let Some(middle) = update.get(update.len() / 2) {
                        middle_number_sum += middle;
                    }
                }
            }
        }
    }

    middle_number_sum
}
