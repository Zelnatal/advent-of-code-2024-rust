use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day19/2.txt").unwrap();
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

fn solver(input: &str) -> i64 {
    let mut lines = input.lines();
    let pattern = lines.next().unwrap().split(", ").collect::<TreeNode>();
    lines.next();
    let towels = lines.collect::<Vec<_>>();
    let mut ans = 0;
    for towel in towels {
        ans += combination(towel, 0, &pattern, &mut HashMap::new())
    }
    ans
}

fn combination(towel: &str, i: usize, pattern: &TreeNode, cache: &mut HashMap<usize, i64>) -> i64 {
    if i >= towel.len() {
        return 1;
    }
    if let Some(out) = cache.get(&i) {
        return *out;
    }
    let mut cur = pattern;
    let mut out = 0;
    for (j, c) in towel.chars().enumerate().skip(i) {
        if let Some(next) = cur.next(c) {
            if next.value {
                out += combination(towel, j + 1, pattern, cache)
            }
            cur = next;
        } else {
            break;
        }
    }
    cache.insert(i, out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day19/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 16)
    }
}
