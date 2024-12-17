use std::collections::{HashMap, HashSet};

#[path = "utils/input_reader.rs"]
mod input_reader;

#[allow(dead_code)]
pub fn part1() -> usize {
    let file_path = "./input/day8.txt";
    let mut antinode_locations: HashSet<String> = HashSet::new();

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut frequency_location_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for line in lines.flatten() {
            let chars = line.chars().collect();
            matrix.push(chars);
        }

        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                let frequency = matrix[x][y];

                if frequency != '.' {
                    frequency_location_map
                        .entry(frequency)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                }
            }
        }

        for pair in frequency_location_map {
            let locations = pair.1;

            for i in 0..locations.len() - 1 {
                let location_i = (locations[i].0 as i32, locations[i].1 as i32);
                for j in i + 1..locations.len() {
                    let location_j = (locations[j].0 as i32, locations[j].1 as i32);
                    let diff = (location_j.0 - location_i.0, location_j.1 - location_i.1);

                    let mut antinode_location = (location_i.0 - diff.0, location_i.1 - diff.1);

                    if antinode_location.0 >= 0
                        && antinode_location.0 < matrix.len() as i32
                        && antinode_location.1 >= 0
                        && antinode_location.1 < matrix[0].len() as i32
                    {
                        antinode_locations.insert(format!("{antinode_location:?}"));
                        let c = matrix[antinode_location.0 as usize][antinode_location.1 as usize];
                        if c == '.' {
                            matrix[antinode_location.0 as usize][antinode_location.1 as usize] =
                                '#';
                        }
                    }

                    antinode_location = (location_j.0 + diff.0, location_j.1 + diff.1);

                    if antinode_location.0 >= 0
                        && antinode_location.0 < matrix.len() as i32
                        && antinode_location.1 >= 0
                        && antinode_location.1 < matrix[0].len() as i32
                    {
                        antinode_locations.insert(format!("{antinode_location:?}"));
                        let c = matrix[antinode_location.0 as usize][antinode_location.1 as usize];
                        if c == '.' {
                            matrix[antinode_location.0 as usize][antinode_location.1 as usize] =
                                '#';
                        }
                    }
                }
            }
        }
    }

    antinode_locations.len()
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let file_path = "./input/day8.txt";
    let mut antinode_locations: HashSet<String> = HashSet::new();

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut frequency_location_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for line in lines.flatten() {
            let chars = line.chars().collect();
            matrix.push(chars);
        }

        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                let frequency = matrix[x][y];

                if frequency != '.' {
                    frequency_location_map
                        .entry(frequency)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                }
            }
        }

        for pair in frequency_location_map {
            let locations = pair.1;

            for i in 0..locations.len() - 1 {
                let location_i = (locations[i].0 as i32, locations[i].1 as i32);
                for j in i + 1..locations.len() {
                    let location_j = (locations[j].0 as i32, locations[j].1 as i32);
                    let diff = (location_j.0 - location_i.0, location_j.1 - location_i.1);

                    antinode_locations.insert(format!("{location_i:?}"));
                    antinode_locations.insert(format!("{location_j:?}"));

                    let mut multiplier = 1;
                    loop {
                        let antinode_location = (
                            location_i.0 - diff.0 * multiplier,
                            location_i.1 - diff.1 * multiplier,
                        );
                        if antinode_location.0 >= 0
                            && antinode_location.0 < matrix.len() as i32
                            && antinode_location.1 >= 0
                            && antinode_location.1 < matrix[0].len() as i32
                        {
                            antinode_locations.insert(format!("{antinode_location:?}"));
                            let c =
                                matrix[antinode_location.0 as usize][antinode_location.1 as usize];
                            if c == '.' {
                                matrix[antinode_location.0 as usize]
                                    [antinode_location.1 as usize] = '#';
                            }
                        } else {
                            break;
                        }

                        multiplier += 1;
                    }

                    multiplier = 1;
                    loop {
                        let antinode_location = (
                            location_j.0 + diff.0 * multiplier,
                            location_j.1 + diff.1 * multiplier,
                        );

                        if antinode_location.0 >= 0
                            && antinode_location.0 < matrix.len() as i32
                            && antinode_location.1 >= 0
                            && antinode_location.1 < matrix[0].len() as i32
                        {
                            antinode_locations.insert(format!("{antinode_location:?}"));
                            let c =
                                matrix[antinode_location.0 as usize][antinode_location.1 as usize];
                            if c == '.' {
                                matrix[antinode_location.0 as usize]
                                    [antinode_location.1 as usize] = '#';
                            }
                        } else {
                            break;
                        }

                        multiplier += 1;
                    }
                }
            }
        }

        for m in matrix {
            println!("{}", String::from_iter(m));
        }
    }

    antinode_locations.len()
}
