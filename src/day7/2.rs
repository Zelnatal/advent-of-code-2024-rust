use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day7/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i64 {
    let equation = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<_>>())
        .map(|v| {
            (
                v[0].parse::<i64>().unwrap(),
                v[1].split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut ans = 0;
    for (expect, nums) in equation {
        let mut next = nums.iter();
        let cur = *next.next().unwrap();
        if evaluate(expect, cur, next) {
            ans += expect
        }
    }
    ans
}

fn evaluate(expect: i64, cur: i64, mut next: std::slice::Iter<'_, i64>) -> bool {
    if let Some(&next_num) = next.next() {
        evaluate(
            expect,
            (format!("{}{}", cur, next_num)).parse().unwrap(),
            next.clone(),
        ) || evaluate(expect, cur + next_num, next.clone())
            || evaluate(expect, cur * next_num, next)
    } else {
        expect == cur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day7/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 11387)
    }
}
