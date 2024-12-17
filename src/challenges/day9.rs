#[path = "utils/input_reader.rs"]
mod input_reader;

#[allow(dead_code)]
pub fn part1() -> u64 {
    let file_path = "./input/day9.txt";
    let mut checksum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut disk_blocks: Vec<Option<u64>> = Vec::new();
        let mut next_file_id: u64 = 0;
        let mut get_file_size = true;

        for line in lines.flatten() {
            for c in line.chars() {
                if get_file_size {
                    let size = c.to_digit(10).unwrap();
                    disk_blocks.extend(std::iter::repeat(Some(next_file_id)).take(size as usize));
                    next_file_id += 1;
                } else {
                    let size = c.to_digit(10).unwrap();
                    disk_blocks.extend(std::iter::repeat(None).take(size as usize));
                }
                get_file_size = !get_file_size;
            }
        }

        let disk_blocks_size = disk_blocks.len();
        let mut free_indices_stack: Vec<usize> = Vec::new();

        for i in (0..disk_blocks_size).rev() {
            if disk_blocks[i].is_none() {
                free_indices_stack.push(i);
            }
        }

        for i in (0..disk_blocks_size).rev() {
            if free_indices_stack.is_empty() {
                break;
            }
            if disk_blocks[i].is_some() {
                let free_index = free_indices_stack.pop();
                let free_index_unwrap = free_index.unwrap();
                if free_index_unwrap < i {
                    disk_blocks[free_index_unwrap] = disk_blocks[i];
                    disk_blocks[i] = None;
                }
            }
        }

        for i in 0..disk_blocks_size {
            if let Some(val) = disk_blocks[i] {
                checksum += (i as u64) * val;
            }
        }
    }

    checksum
}

#[allow(dead_code)]
pub fn part2() -> u64 {
    let file_path = "./input/day9.txt";
    let mut checksum = 0;

    if let Ok(lines) = input_reader::read_lines(file_path) {
        let mut disk_blocks: Vec<Option<u64>> = Vec::new();
        let mut next_file_id: u64 = 0;
        let mut get_file_size = true;
        let mut free_blocks: Vec<(usize, u32)> = Vec::new();
        let mut file_blocks: Vec<(u64, usize, u32)> = Vec::new();

        for line in lines.flatten() {
            for c in line.chars() {
                if get_file_size {
                    let size = c.to_digit(10).unwrap();
                    file_blocks.push((next_file_id, disk_blocks.len(), size));
                    disk_blocks.extend(std::iter::repeat(Some(next_file_id)).take(size as usize));
                    next_file_id += 1;
                } else {
                    let size = c.to_digit(10).unwrap();
                    free_blocks.push((disk_blocks.len(), size));
                    disk_blocks.extend(std::iter::repeat(None).take(size as usize));
                }
                get_file_size = !get_file_size;
            }
        }

        file_blocks.sort_by(|a, b| b.0.cmp(&(a.0)));

        let disk_blocks_size = disk_blocks.len();
        let mut free_indices: Vec<usize> = Vec::new();

        for i in 0..disk_blocks_size {
            if disk_blocks[i].is_none() {
                free_indices.push(i);
            }
        }

        let file_blocks_size = file_blocks.len();
        for i in 0..file_blocks_size {
            if free_indices.is_empty() {
                break;
            }

            let fb = file_blocks[i];
            let file_size = fb.2 as usize;
            let file_start_index = fb.1;

            if file_size > free_indices.len() {
                continue;
            }
            let file_id = fb.0;

            for j in 0..free_indices.len() - file_size {
                let slice = &free_indices[j..j + file_size];
                let start_index = slice[0];

                if start_index > file_start_index {
                    break;
                }

                let mut enough_free_space = true;
                for k in 0..slice.len() {
                    enough_free_space = slice[k] == start_index + k;
                    if !enough_free_space {
                        break;
                    }
                }

                if enough_free_space {
                    for k in start_index..start_index + file_size {
                        disk_blocks[k] = Some(file_id);
                    }

                    for k in file_start_index..file_start_index + file_size {
                        disk_blocks[k] = None;
                    }

                    for _ in j..j + file_size {
                        free_indices.remove(j);
                    }
                    for k in j..j + file_size {
                        free_indices.push(file_start_index + k);
                    }

                    free_indices.sort();
                    break;
                }
            }
        }

        for i in 0..disk_blocks_size {
            if let Some(val) = disk_blocks[i] {
                checksum += (i as u64) * val;
            }
        }
    }

    checksum
}
