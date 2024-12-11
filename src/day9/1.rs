use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day9/1.txt").unwrap();
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
    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        if disk[left].is_none() {
            disk.swap(left, right);
            right -= 1;
        } else {
            left += 1;
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
        let input = read_to_string("./input/day9/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 1928)
    }
}
