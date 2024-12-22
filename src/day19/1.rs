use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day19/1.txt").unwrap();
    println!("{}", solver(&input))
}

#[derive(Debug)]
struct TreeNode {
    children: HashMap<char, TreeNode>,
    value: bool,
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: false,
        }
    }

    fn insert(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        let mut chars = s.chars();
        let mut next = self.children.entry(chars.next().unwrap()).or_default();
        for c in chars {
            next = next.children.entry(c).or_default()
        }
        next.value = true
    }

    fn next(&self, c: char) -> Option<&TreeNode> {
        self.children.get(&c)
    }
}

impl<'a> FromIterator<&'a str> for TreeNode {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut node = TreeNode::new();
        for s in iter {
            node.insert(s);
        }
        node
    }
}

fn solver(input: &str) -> i32 {
    let mut lines = input.lines();
    let pattern = lines.next().unwrap().split(", ").collect::<TreeNode>();
    lines.next();
    let towels = lines.collect::<Vec<_>>();
    let mut ans = 0;
    for towel in towels {
        ans += i32::from(is_possible(towel, 0, &pattern, &mut HashSet::new()))
    }
    ans
}

fn is_possible(towel: &str, i: usize, pattern: &TreeNode, not_possible: &mut HashSet<usize>) -> bool {
    if not_possible.contains(&i) {
        return false;
    }
    if i >= towel.len() {
        return true;
    }
    let mut cur = pattern;
    for (j, c) in towel.chars().enumerate().skip(i) {
        if let Some(next) = cur.next(c) {
            if next.value && is_possible(towel, j+1, pattern, not_possible) {
                return true;
            }
            cur = next;
        } else {
            break;
        }
    }
    not_possible.insert(i);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day19/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 6)
    }
}
