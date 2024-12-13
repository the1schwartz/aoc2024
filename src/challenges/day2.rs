#[path = "utils/input_reader.rs"]
mod input_reader;

enum LevelDirection {
    Increasing,
    Decreasing,
}

fn check_level(direction: &Option<LevelDirection>, prev: i32, current: i32) -> bool {
    if current == prev {
        return false;
    }

    let diff = current - prev;

    if diff > 0 {
        if let Some(d) = direction {
            let is_descreasing = match d {
                LevelDirection::Increasing => false,
                LevelDirection::Decreasing => true,
            };
            if diff > 3 || is_descreasing {
                return false;
            }
        }
    } else {
        if let Some(d) = direction {
            let is_increasing = match d {
                LevelDirection::Increasing => true,
                LevelDirection::Decreasing => false,
            };
            if diff < -3 || is_increasing {
                return false;
            }
        }
    }

    true
}

fn check_whole_level(values: &Vec<i32>) -> bool {
    let mut prev_value: Option<i32> = None;
    let mut report_safe = true;
    let mut direction: Option<LevelDirection> = None;

    for v in values {
        if let Some(prev) = prev_value {
            if direction.is_none() {
                if *v > prev {
                    direction = Some(LevelDirection::Increasing);
                } else if *v < prev {
                    direction = Some(LevelDirection::Decreasing);
                }
            }

            report_safe = check_level(&direction, prev, *v);

            if !report_safe {
                break;
            }

            prev_value = Some(*v);
        } else {
            prev_value = Some(*v);
        }
    }

    report_safe
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file_path = "./input/day2.txt";
    let mut total_safe_reports = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut values: Vec<i32> = Vec::new();

            for p in parts {
                let value: i32 = p.parse().unwrap();
                values.push(value);
            }

            let report_safe = check_whole_level(&values);

            if report_safe {
                total_safe_reports += 1;
            }
        }

        println!("Total safe reports: {}", total_safe_reports);
    }

    total_safe_reports
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file_path = "./input/day2.txt";
    let mut total_safe_reports = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut values: Vec<i32> = Vec::new();

            for p in parts {
                let value: i32 = p.parse().unwrap();
                values.push(value);
            }

            let mut report_safe = check_whole_level(&values);

            if !report_safe {
                for i in 0..values.len() {
                    let mut copy = values.clone();
                    copy.remove(i);

                    report_safe = check_whole_level(&copy);

                    if report_safe {
                        break;
                    }
                }
            }

            if report_safe {
                total_safe_reports += 1;
            }
        }

        println!("Total safe reports: {}", total_safe_reports);
    }

    total_safe_reports
}
