use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day5/2.txt").unwrap();
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

    for line in lines {
        let mut vec = line.split(',').collect::<Vec<_>>();
        let mut seen = HashSet::new();
        let pages = vec.iter().copied().collect::<HashSet<_>>();
        let mut i = 0;
        let mut correct = true;
        while i < vec.len() {
            let page = vec[i];
            for &prev_page in rules.get(page).unwrap_or(&vec![]) {
                if pages.contains(prev_page) && !seen.contains(prev_page) {
                    ord(prev_page, &mut vec, &mut i, &mut seen, &pages, &rules);
                    correct = false;
                }
            }
            if !seen.insert(page) {
                vec.remove(i);
            } else {
                i += 1;
            }
        }
        if !correct {
            ans += vec[vec.len() / 2].parse::<i32>().unwrap();
        }
    }
    ans
}

fn ord<'a>(
    page: &'a str,
    vec: &mut Vec<&'a str>,
    i: &mut usize,
    seen: &mut HashSet<&'a str>,
    pages: &HashSet<&str>,
    rules: &HashMap<&str, Vec<&'a str>>,
) {
    for &prev_page in rules.get(page).unwrap_or(&vec![]) {
        if pages.contains(prev_page) && !seen.contains(prev_page) {
            ord(prev_page, vec, i, seen, pages, rules)
        }
    }
    vec.insert(*i, page);
    seen.insert(page);
    *i += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day5/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 123)
    }
}
