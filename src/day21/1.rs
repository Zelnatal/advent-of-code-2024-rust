use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day21/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let codes = input.lines().collect::<Vec<_>>();
    let mut ans = 0;
    for code in codes {
        let keys = keyword(code);
        ans += code[..code.len() - 1].parse::<i32>().unwrap() * keys.len() as i32
    }
    ans
}

fn keyword(code: &str) -> String {
    let mut out = Vec::new();
    let mut keyboard = Keyboard::new();
    for d in code.chars() {
        if out.is_empty() {
            for nk in keyboard.next_pos(d) {
                for p in directional1(&nk) {
                    out.push(p);
                }
            }
        } else {
            let mut new_out = Vec::new();
            for nk in keyboard.next_pos(d) {
                for o in out.iter() {
                    for p in directional1(&nk) {
                        new_out.push(format!("{}{}", o, p));
                    }
                }
            }
            out = new_out
        }
    }
    out.sort_by_key(|s|s.len());
    out[0].clone()
}

fn directional1(keys: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut keyboard = Direcional::new();
    for d in keys.chars() {
        if out.is_empty() {
            for nk in keyboard.next_pos(d) {
                for p in directional2(&nk) {
                    out.push(p);
                }
            }
        } else {
            let mut new_out = Vec::new();
            for nk in keyboard.next_pos(d) {
                for p in directional2(&nk) {
                    for o in out.iter() {
                        new_out.push(format!("{}{}", o, p));
                    }
                }
            }
            out = new_out
        }
    }
    out
}

fn directional2(keys: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut direction = Direcional::new();
    for d in keys.chars() {
        if out.is_empty() {
            for p in direction.next_pos(d) {
                out.push(p);
            }
        } else {
            let mut new_out = Vec::new();
            for p in direction.next_pos(d) {
                for o in out.iter() {
                    new_out.push(format!("{}{}", o, p));
                }
            }
            out = new_out
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day21/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 126384)
    }
}
