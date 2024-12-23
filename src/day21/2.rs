use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day21/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i64 {
    let codes = input.lines().collect::<Vec<_>>();
    let mut ans = 0;
    let mut cache = HashMap::new();
    for code in codes {
        let keys = keyword(code, &mut cache);
        ans += code[..code.len() - 1].parse::<i64>().unwrap() * keys
    }
    ans
}

fn keyword(code: &str, cache: &mut HashMap<(String, i32), i64>) -> i64 {
    let mut out = i64::MAX;
    let mut keyboard = Keyboard::new();
    for d in code.chars() {
        if out == i64::MAX {
            for nk in keyboard.next_pos(d) {
                let new_out = directional(&nk, 25, cache);
                out = out.min(new_out)
            }
        } else {
            let o = out;
            out = i64::MAX;
            for nk in keyboard.next_pos(d) {
                let new_out = o + directional(&nk, 25, cache);
                out = out.min(new_out)
            }
        }
    }
    out
}

fn directional(keys: &str, cur: i32, cache: &mut HashMap<(String, i32), i64>) -> i64 {
    if let Some(out) = cache.get(&(keys.to_owned(), cur)) {
        return *out;
    }
    let mut out = i64::MAX;
    let mut keyboard = Direcional::new();
    if cur > 1 {
        for d in keys.chars() {
            if out == i64::MAX {
                for nk in keyboard.next_pos(d) {
                    let new_out = directional(&nk, cur - 1, cache);
                    out = out.min(new_out)
                }
            } else {
                let o = out;
                out = i64::MAX;
                for nk in keyboard.next_pos(d) {
                    let new_out = o + directional(&nk, cur - 1, cache);
                    out = out.min(new_out)
                }
            }
        }
    } else {
        for d in keys.chars() {
            if out == i64::MAX {
                for p in keyboard.next_pos(d) {
                    let new_out = p.len() as i64;
                    out = out.min(new_out)
                }
            } else {
                let o = out;
                out = i64::MAX;
                for p in keyboard.next_pos(d) {
                    let new_out = o + p.len() as i64;
                    out = out.min(new_out)
                }
            }
        }
    }

    cache.insert((keys.to_owned(), cur), out);

    out
}

const DIRECTION: [isize; 5] = [-1, 0, 1, 0, -1];

#[derive(Debug, Clone)]
struct Direcional {
    x: isize,
    y: isize,
}

impl Direcional {
    fn new() -> Self {
        Self { x: 0, y: 2 }
    }

    fn next_pos(&mut self, c: char) -> Vec<String> {
        let target = Self::key_pos(c);
        let mut out = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.x, self.y, String::new()));
        while let Some((x, y, mut keys)) = queue.pop_front() {
            if (x, y) == target {
                keys.push('A');
                out.push(keys);
                continue;
            }

            let cur_dist = Self::get_dist((x, y), target);

            for (d, dc) in ['^', '>', 'v', '<'].into_iter().enumerate() {
                let nx = x + DIRECTION[d];
                let ny = y + DIRECTION[d + 1];
                if Self::is_valid((nx, ny)) && Self::get_dist((nx, ny), target) < cur_dist {
                    let mut new_key = keys.clone();
                    new_key.push(dc);
                    queue.push_back((nx, ny, new_key));
                }
            }
        }
        self.x = target.0;
        self.y = target.1;
        out
    }

    fn key_pos(c: char) -> (isize, isize) {
        match c {
            '^' => (0, 1),
            'A' => (0, 2),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            _ => unreachable!(),
        }
    }

    fn get_dist(from: (isize, isize), target: (isize, isize)) -> isize {
        (from.0 - target.0).pow(2) + (from.1 - target.1).pow(2)
    }

    fn is_valid(target: (isize, isize)) -> bool {
        (0..=1).contains(&target.0) && (0..=2).contains(&target.1) && target != (0, 0)
    }
}

#[derive(Debug, Clone)]
struct Keyboard {
    x: isize,
    y: isize,
}

impl Keyboard {
    fn new() -> Self {
        Self { x: 3, y: 2 }
    }

    fn next_pos(&mut self, c: char) -> Vec<String> {
        let target = Self::key_pos(c);
        let mut out = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.x, self.y, String::new()));
        while let Some((x, y, mut keys)) = queue.pop_front() {
            if (x, y) == target {
                keys.push('A');
                out.push(keys);
                continue;
            }

            let cur_dist = Self::get_dist((x, y), target);

            for (d, dc) in ['^', '>', 'v', '<'].into_iter().enumerate() {
                let nx = x + DIRECTION[d];
                let ny = y + DIRECTION[d + 1];
                if Self::is_valid((nx, ny)) && Self::get_dist((nx, ny), target) < cur_dist {
                    let mut new_key = keys.clone();
                    new_key.push(dc);
                    queue.push_back((nx, ny, new_key));
                }
            }
        }
        self.x = target.0;
        self.y = target.1;
        out
    }

    fn key_pos(c: char) -> (isize, isize) {
        match c {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => unreachable!(),
        }
    }

    fn get_dist(from: (isize, isize), target: (isize, isize)) -> isize {
        (from.0 - target.0).pow(2) + (from.1 - target.1).pow(2)
    }

    fn is_valid(target: (isize, isize)) -> bool {
        (0..=3).contains(&target.0) && (0..=2).contains(&target.1) && target != (3, 0)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exemple() {
//         let input = read_to_string("./input/day20/2_exemple.txt").unwrap();
//         assert_eq!(solver(&input), 126384)
//     }
// }
