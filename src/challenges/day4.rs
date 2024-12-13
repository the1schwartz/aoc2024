#[path = "utils/input_reader.rs"]
mod input_reader;

fn get_element(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }

    if let Some(x_val) = matrix.get(x as usize) {
        if let Some(y_val) = x_val.get(y as usize) {
            return Some(*y_val);
        }
    }

    return None;
}

#[allow(dead_code)]
pub fn part1() -> i32 {
    let file_path = "./input/day4.txt";
    let mut xmas_count = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in lines.flatten() {
            let chars = line.chars().collect();
            matrix.push(chars);
        }

        for xx in 0..matrix.len() {
            for yy in 0..matrix[xx].len() {
                let c0 = matrix[xx][yy];
                let x = xx as i32;
                let y = yy as i32;

                if c0 == 'X' {
                    // Horizontal: left to right
                    let mut c1 = get_element(&matrix, x, y + 1);
                    let mut c2 = get_element(&matrix, x, y + 2);
                    let mut c3 = get_element(&matrix, x, y + 3);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Horizontal: right to left
                    c1 = get_element(&matrix, x, y - 1);
                    c2 = get_element(&matrix, x, y - 2);
                    c3 = get_element(&matrix, x, y - 3);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Vertical: top to bottom
                    c1 = get_element(&matrix, x + 1, y);
                    c2 = get_element(&matrix, x + 2, y);
                    c3 = get_element(&matrix, x + 3, y);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Vertical: bottom to top
                    c1 = get_element(&matrix, x - 1, y);
                    c2 = get_element(&matrix, x - 2, y);
                    c3 = get_element(&matrix, x - 3, y);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Diagonal: top to bottom-right
                    c1 = get_element(&matrix, x + 1, y + 1);
                    c2 = get_element(&matrix, x + 2, y + 2);
                    c3 = get_element(&matrix, x + 3, y + 3);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Diagonal: bottom to top-left
                    c1 = get_element(&matrix, x - 1, y - 1);
                    c2 = get_element(&matrix, x - 2, y - 2);
                    c3 = get_element(&matrix, x - 3, y - 3);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Diagonal: bottom to top-right
                    c1 = get_element(&matrix, x - 1, y + 1);
                    c2 = get_element(&matrix, x - 2, y + 2);
                    c3 = get_element(&matrix, x - 3, y + 3);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }

                    // Diagonal: top to bottom-left
                    c1 = get_element(&matrix, x + 1, y - 1);
                    c2 = get_element(&matrix, x + 2, y - 2);
                    c3 = get_element(&matrix, x + 3, y - 3);

                    if c1.is_some_and(|c| c == 'M')
                        && c2.is_some_and(|c| c == 'A')
                        && c3.is_some_and(|c| c == 'S')
                    {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count
}

#[allow(dead_code)]
pub fn part2() -> i32 {
    let file_path = "./input/day4.txt";
    let mut xmas_count = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        for line in lines.flatten() {
            let chars = line.chars().collect();
            matrix.push(chars);
        }

        for xx in 0..matrix.len() {
            for yy in 0..matrix[xx].len() {
                let c0 = matrix[xx][yy];
                let x = xx as i32;
                let y = yy as i32;

                if c0 == 'A' {
                    let mut part_count = 0;

                    // Diagonal: top to bottom-right
                    let mut c1 = get_element(&matrix, x - 1, y - 1);
                    let mut c2 = get_element(&matrix, x + 1, y + 1);

                    if c1.is_some_and(|c| c == 'M') && c2.is_some_and(|c| c == 'S') {
                        part_count += 1;
                    }

                    // Diagonal: bottom to top-left
                    c1 = get_element(&matrix, x + 1, y + 1);
                    c2 = get_element(&matrix, x - 1, y - 1);

                    if c1.is_some_and(|c| c == 'M') && c2.is_some_and(|c| c == 'S') {
                        part_count += 1;
                    }

                    // Diagonal: bottom to top-right
                    c1 = get_element(&matrix, x + 1, y - 1);
                    c2 = get_element(&matrix, x - 1, y + 1);

                    if c1.is_some_and(|c| c == 'M') && c2.is_some_and(|c| c == 'S') {
                        part_count += 1;
                    }

                    // Diagonal: top to bottom-left
                    c1 = get_element(&matrix, x - 1, y + 1);
                    c2 = get_element(&matrix, x + 1, y - 1);

                    if c1.is_some_and(|c| c == 'M') && c2.is_some_and(|c| c == 'S') {
                        part_count += 1;
                    }

                    if part_count == 2 {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count
}
