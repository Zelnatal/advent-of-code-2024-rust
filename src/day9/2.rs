use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day9/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let mut disk = vec![];
    let mut is_file = true;
    let mut id = 0;
    for c in input.chars() {
        for _ in 0..c.to_digit(10).unwrap() {
            if is_file {
                disk.push(Some(id));
            } else {
                disk.push(None);
            }
        }
        id += i32::from(is_file);
        is_file = !is_file;
    }

    let mut right = disk.len() as isize - 1;
    let mut moved = HashSet::new();

    while right >= 0 {
        if disk[right as usize].is_some() {
            let mut len = 1;
            while right - len >= 0 && disk[(right - len) as usize] == disk[right as usize] {
                len += 1;
            }
            if moved.contains(&disk[right as usize].unwrap()) {
                right -= len;
                continue;
            }
            let mut left = 0;
            let mut found_free = false;
            'found_free: while left < right {
                if disk[left as usize].is_none() {
                    let mut free_len = 1;
                    while left + free_len < right && disk[(left + free_len) as usize].is_none() {
                        free_len += 1;
                    }
                    if free_len >= len {
                        found_free = true;
                        break 'found_free;
                    }
                    left += free_len;
                } else {
                    left += 1;
                }
            }

            if found_free {
                moved.insert(disk[right as usize].unwrap());
                for i in 0..len {
                    disk.swap((right - i) as usize, (left + i) as usize);
                }
            }
            right -= len;
        } else {
            right -= 1;
        }
    }
    disk.into_iter()
        .enumerate()
        .map(|(i, n)| i * n.unwrap_or(0) as usize)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day9/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 2858)
    }
}
