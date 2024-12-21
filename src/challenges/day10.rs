use std::collections::HashSet;

#[path = "utils/input_reader.rs"]
mod input_reader;

#[allow(dead_code)]
pub fn part1() -> u32 {
    let file_path = "./input/day10.txt";
    let mut total_trailhead_score_sum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut map: Vec<Vec<u32>> = Vec::new();

        for line in lines.flatten() {
            let digits = line.chars().filter_map(|s| s.to_digit(10)).collect();
            map.push(digits);
        }

        let mut trailheads: Vec<(usize, usize)> = Vec::new();

        for x in 0..map.len() {
            let row = &map[x];
            for y in 0..row.len() {
                let h = row[y];
                if h == 0 {
                    trailheads.push((x, y));
                }
            }
        }

        println!("Number of trailhead: {}", trailheads.len());

        for th in trailheads {
            let mut stack: Vec<(usize, usize)> = Vec::new();
            let mut trailhead_score = 0;
            let mut visited_ends: HashSet<String> = HashSet::new();

            stack.push(th);

            while !stack.is_empty() {
                let coords = stack.pop().unwrap();
                let h = map[coords.0][coords.1];

                if h == 9 {
                    let end_key = format!("{coords:?}");

                    if !visited_ends.contains(&end_key) {
                        visited_ends.insert(end_key);
                        trailhead_score += 1;
                    }
                    continue;
                }

                let coords: (i32, i32) = (coords.0 as i32, coords.1 as i32);
                let next_h = h + 1;

                let up_coords = (coords.0 - 1, coords.1);
                if up_coords.0 >= 0 {
                    let up_coords = (up_coords.0 as usize, up_coords.1 as usize);
                    let up_h = map[up_coords.0][up_coords.1];
                    if up_h == next_h {
                        stack.push(up_coords);
                    }
                }

                let down_coords = (coords.0 + 1, coords.1);
                if down_coords.0 < map.len() as i32 {
                    let down_coords = (down_coords.0 as usize, down_coords.1 as usize);
                    let down_h = map[down_coords.0][down_coords.1];
                    if down_h == next_h {
                        stack.push(down_coords);
                    }
                }

                let left_coords = (coords.0, coords.1 - 1);
                if left_coords.1 >= 0 {
                    let left_coords = (left_coords.0 as usize, left_coords.1 as usize);
                    let left_h = map[left_coords.0][left_coords.1];
                    if left_h == next_h {
                        stack.push(left_coords);
                    }
                }

                let right_coords = (coords.0, coords.1 + 1);
                if right_coords.1 < map[0].len() as i32 {
                    let right_coords = (right_coords.0 as usize, right_coords.1 as usize);
                    let right_h = map[right_coords.0][right_coords.1];
                    if right_h == next_h {
                        stack.push(right_coords);
                    }
                }
            }

            total_trailhead_score_sum += trailhead_score;
        }
    }
    total_trailhead_score_sum
}

#[allow(dead_code)]
pub fn part2() -> u32 {
    let file_path = "./input/day10.txt";
    let mut total_trailhead_score_sum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut map: Vec<Vec<u32>> = Vec::new();

        for line in lines.flatten() {
            let digits = line.chars().filter_map(|s| s.to_digit(10)).collect();
            map.push(digits);
        }

        let mut trailheads: Vec<(usize, usize)> = Vec::new();

        for x in 0..map.len() {
            let row = &map[x];
            for y in 0..row.len() {
                let h = row[y];
                if h == 0 {
                    trailheads.push((x, y));
                }
            }
        }

        println!("Number of trailhead: {}", trailheads.len());

        for th in trailheads {
            let mut stack: Vec<(usize, usize)> = Vec::new();
            let mut trailhead_score = 0;

            stack.push(th);

            while !stack.is_empty() {
                let coords = stack.pop().unwrap();
                let h = map[coords.0][coords.1];

                if h == 9 {
                    trailhead_score += 1;
                    continue;
                }

                let coords: (i32, i32) = (coords.0 as i32, coords.1 as i32);
                let next_h = h + 1;

                let up_coords = (coords.0 - 1, coords.1);
                if up_coords.0 >= 0 {
                    let up_coords = (up_coords.0 as usize, up_coords.1 as usize);
                    let up_h = map[up_coords.0][up_coords.1];
                    if up_h == next_h {
                        stack.push(up_coords);
                    }
                }

                let down_coords = (coords.0 + 1, coords.1);
                if down_coords.0 < map.len() as i32 {
                    let down_coords = (down_coords.0 as usize, down_coords.1 as usize);
                    let down_h = map[down_coords.0][down_coords.1];
                    if down_h == next_h {
                        stack.push(down_coords);
                    }
                }

                let left_coords = (coords.0, coords.1 - 1);
                if left_coords.1 >= 0 {
                    let left_coords = (left_coords.0 as usize, left_coords.1 as usize);
                    let left_h = map[left_coords.0][left_coords.1];
                    if left_h == next_h {
                        stack.push(left_coords);
                    }
                }

                let right_coords = (coords.0, coords.1 + 1);
                if right_coords.1 < map[0].len() as i32 {
                    let right_coords = (right_coords.0 as usize, right_coords.1 as usize);
                    let right_h = map[right_coords.0][right_coords.1];
                    if right_h == next_h {
                        stack.push(right_coords);
                    }
                }
            }

            total_trailhead_score_sum += trailhead_score;
        }
    }
    total_trailhead_score_sum
}
