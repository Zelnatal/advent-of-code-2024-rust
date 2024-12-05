use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day2/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut ans = 0;
    for line in input.lines() {
        let levels = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        ans += i32::from(remove(levels))
    }
    ans
}

fn remove(mut arr: Vec<i32>) -> bool {
    if (valid(&arr, 1)) || (valid(&arr, -1)) {
        return true;
    }
    for i in 0..arr.len() {
        let n = arr.remove(i);
        if (valid(&arr, 1)) || (valid(&arr, -1)) {
            return true;
        }
        arr.insert(i, n);
    }
    false
}

fn valid(arr: &[i32], invert: i32) -> bool {
    for nums in arr.windows(2) {
        if !(1..=3).contains(&((nums[0] - nums[1]) * invert)) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day2/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 4)
    }
}
