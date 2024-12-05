use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day2/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut ans = 0;
    'line: for line in input.lines() {
        let levels = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if levels[0] > levels[1] {
            for nums in levels.windows(2) {
                if !(1..=3).contains(&(nums[0] - nums[1])) {
                    continue 'line;
                }
            }
            ans += 1;
        } else {
            for nums in levels.windows(2) {
                if !(1..=3).contains(&(nums[1] - nums[0])) {
                    continue 'line;
                }
            }
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day2/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 2)
    }
}
