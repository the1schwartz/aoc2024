use std::collections::HashMap;

#[path = "utils/input_reader.rs"]
mod input_reader;

fn find_guard_position(matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut guard_position: Option<(usize, usize)> = None;

    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            let c = matrix[x][y];

            if c == '^' {
                guard_position = Some((x, y));
                break;
            }
        }

        if guard_position.is_some() {
            break;
        }
    }

    guard_position
}

fn mark_guard_positions(
    mut matrix: Vec<Vec<char>>,
    mut guard_position: (usize, usize),
) -> Vec<Vec<char>> {
    let mut guard_direction = (-1, 0);

    loop {
        matrix[guard_position.0][guard_position.1] = 'X';

        let new_guard_position = (
            guard_position.0 as i32 + guard_direction.0,
            guard_position.1 as i32 + guard_direction.1,
        );

        if new_guard_position.0 < 0
            || new_guard_position.1 < 0
            || new_guard_position.0 >= matrix.len() as i32
            || new_guard_position.1 >= matrix[0].len() as i32
        {
            break;
        }

        let new_guard_position = (new_guard_position.0 as usize, new_guard_position.1 as usize);

        if matrix[new_guard_position.0][new_guard_position.1] == '#' {
            guard_direction = (guard_direction.1, guard_direction.0 * -1);
        } else {
            guard_position = new_guard_position;
        }
    }

    matrix
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file_path = "./input/day6.txt";
    let mut total_distinct_positions = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in lines.flatten() {
            let chars = line.chars().collect();
            matrix.push(chars);
        }

        let guard_position: Option<(usize, usize)> = find_guard_position(&matrix);
        let guard_position = guard_position.unwrap();

        matrix = mark_guard_positions(matrix, guard_position);

        for v in &matrix {
            for c in v {
                if *c == 'X' {
                    total_distinct_positions += 1;
                }
            }
        }
    }

    total_distinct_positions
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file_path = "./input/day6.txt";
    let mut total_stuck_in_loop = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in lines.flatten() {
            let chars = line.chars().collect();
            matrix.push(chars);
        }

        let guard_position: Option<(usize, usize)> = find_guard_position(&matrix);

        let mut guard_position = guard_position.unwrap();
        let guard_start_position = guard_position;

        matrix = mark_guard_positions(matrix, guard_position);

        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                let c = matrix[x][y];

                if c == 'X' && !(x == guard_start_position.0 && y == guard_start_position.1) {
                    let mut matrix_copy = matrix.clone();
                    matrix_copy[x][y] = 'O';

                    guard_position = guard_start_position.clone();
                    let mut guard_direction = (-1, 0);
                    let mut visited_positions: HashMap<String, Vec<char>> = HashMap::new();

                    loop {
                        if guard_direction.0 > 0 {
                            if let Some(p) = visited_positions.get(&format!("{:?}", guard_position))
                            {
                                if p.contains(&'V') {
                                    total_stuck_in_loop += 1;
                                    break;
                                }
                            }
                            matrix_copy[guard_position.0][guard_position.1] = 'V';

                            visited_positions
                                .entry(format!("{:?}", guard_position))
                                .or_insert_with(Vec::new)
                                .push('V');
                        }
                        if guard_direction.0 < 0 {
                            if let Some(p) = visited_positions.get(&format!("{:?}", guard_position))
                            {
                                if p.contains(&'^') {
                                    total_stuck_in_loop += 1;
                                    break;
                                }
                            }
                            matrix_copy[guard_position.0][guard_position.1] = '^';
                            visited_positions
                                .entry(format!("{:?}", guard_position))
                                .or_insert_with(Vec::new)
                                .push('^');
                        }
                        if guard_direction.1 > 0 {
                            if let Some(p) = visited_positions.get(&format!("{:?}", guard_position))
                            {
                                if p.contains(&'>') {
                                    total_stuck_in_loop += 1;
                                    break;
                                }
                            }
                            matrix_copy[guard_position.0][guard_position.1] = '>';
                            visited_positions
                                .entry(format!("{:?}", guard_position))
                                .or_insert_with(Vec::new)
                                .push('>');
                        }
                        if guard_direction.1 < 0 {
                            if let Some(p) = visited_positions.get(&format!("{:?}", guard_position))
                            {
                                if p.contains(&'<') {
                                    total_stuck_in_loop += 1;
                                    break;
                                }
                            }
                            matrix_copy[guard_position.0][guard_position.1] = '<';
                            visited_positions
                                .entry(format!("{:?}", guard_position))
                                .or_insert_with(Vec::new)
                                .push('<');
                        }

                        let new_guard_position = (
                            guard_position.0 as i32 + guard_direction.0,
                            guard_position.1 as i32 + guard_direction.1,
                        );

                        if new_guard_position.0 < 0
                            || new_guard_position.1 < 0
                            || new_guard_position.0 >= matrix_copy.len() as i32
                            || new_guard_position.1 >= matrix_copy[0].len() as i32
                        {
                            break;
                        }

                        let new_guard_position =
                            (new_guard_position.0 as usize, new_guard_position.1 as usize);

                        if matrix_copy[new_guard_position.0][new_guard_position.1] == '#'
                            || matrix_copy[new_guard_position.0][new_guard_position.1] == 'O'
                        {
                            guard_direction = (guard_direction.1, guard_direction.0 * -1);
                        } else {
                            guard_position = new_guard_position;
                        }
                    }
                }
            }
        }
    }

    total_stuck_in_loop
}
