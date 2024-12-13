#[path = "utils/input_reader.rs"]
mod input_reader;

fn to_ternary(mut number: u32, num_of_chars: usize) -> String {
    if number == 0 {
        let character = '0';
        return character.to_string().repeat(num_of_chars);
    }

    let mut ternary = Vec::new();

    while number > 0 {
        ternary.push((number % 3).to_string()); // Get the remainder
        number /= 3; // Divide by 3
    }

    ternary.reverse(); // Reverse to get the correct order

    let mut result = ternary.join("");
    if result.len() < num_of_chars {
        let character = '0';
        let padded_zeros = character.to_string().repeat(num_of_chars - result.len());
        result = format!("{}{}", padded_zeros, result)
    }
    return result;
}

#[allow(dead_code)]
pub fn part1() -> u64 {
    let file_path = "./input/day7.txt";
    let mut total_calibration_result = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        for line in lines.flatten() {
            let mut result: Vec<u64> = line
                .replace(":", "")
                .split_whitespace()
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
            let numbers = result.split_off(1);

            let base: i32 = 2;
            let num_of_chars = numbers.len() - 1;
            let total_combinations = base.pow(num_of_chars as u32);

            let mut found_combintation = false;
            for i in 0..total_combinations {
                let binary_string = format!("{:0width$b}", i, width = num_of_chars);
                let mut binary_chars = binary_string.chars();

                let mut combination_result = numbers[0];
                for j in 0..num_of_chars {
                    if let Some(b) = binary_chars.nth(0) {
                        if b == '0' {
                            combination_result += numbers[j + 1];
                        } else if b == '1' {
                            combination_result *= numbers[j + 1];
                        }
                    }
                }

                if result[0] == combination_result {
                    found_combintation = true;
                    break;
                }
            }

            if found_combintation {
                total_calibration_result += result[0];
            }
        }
    }

    total_calibration_result
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let file_path = "./input/day7.txt";
    let mut total_calibration_result = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        for line in lines.flatten() {
            let mut result: Vec<u64> = line
                .replace(":", "")
                .split_whitespace()
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
            let numbers = result.split_off(1);

            let base: u64 = 3;
            let num_of_chars = numbers.len() - 1;
            let total_combinations = base.pow(num_of_chars as u32);

            let mut found_combintation = false;
            for i in 0..total_combinations {
                let ternary_string = format!("{}", to_ternary(i as u32, num_of_chars));
                let mut ternary_chars = ternary_string.chars();

                let mut combination_result = numbers[0];
                for j in 0..num_of_chars {
                    if let Some(b) = ternary_chars.nth(0) {
                        if b == '0' {
                            combination_result += numbers[j + 1];
                        } else if b == '1' {
                            combination_result *= numbers[j + 1];
                        } else if b == '2' {
                            let concat = format!("{}{}", combination_result, numbers[j + 1]);
                            combination_result = concat.parse().unwrap();
                        }
                    }
                }

                if result[0] == combination_result {
                    found_combintation = true;
                    break;
                }
            }

            if found_combintation {
                total_calibration_result += result[0];
            }
        }
    }

    total_calibration_result
}
