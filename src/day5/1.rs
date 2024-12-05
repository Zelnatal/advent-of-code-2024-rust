use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day5/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut ans = 0;
    let mut rules = HashMap::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let nums = line.split('|').collect::<Vec<_>>();
        rules
            .entry(nums[1])
            .and_modify(|v: &mut Vec<&str>| v.push(nums[0]))
            .or_insert(vec![nums[0]]);
    }

    'line: for line in lines {
        let vec = line.split(',').collect::<Vec<_>>();
        let mut seen = HashSet::new();
        let pages = vec.iter().copied().collect::<HashSet<_>>();
        for &page in vec.iter() {
            for &prev_page in rules.get(page).unwrap_or(&vec![]) {
                if pages.contains(prev_page) && !seen.contains(prev_page) {
                    continue 'line;
                }
            }
            seen.insert(page);
        }
        ans += vec[vec.len() / 2].parse::<i32>().unwrap();
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day5/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 143)
    }
}
