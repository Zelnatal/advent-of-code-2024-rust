use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day13/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let mut machines = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        skip_at(&mut chars, '+');
        chars.next();
        let ax = chars
            .clone()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        skip_at(&mut chars, '+');
        chars.next();
        let ay = chars
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        let line = lines.next().unwrap();
        chars = line.chars();
        skip_at(&mut chars, '+');
        chars.next();
        let bx = chars
            .clone()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        skip_at(&mut chars, '+');
        chars.next();
        let by = chars
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        let line = lines.next().unwrap();
        chars = line.chars();
        skip_at(&mut chars, '=');
        chars.next();
        let px = chars
            .clone()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();
        skip_at(&mut chars, '=');
        chars.next();
        let py = chars
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        lines.next();
        machines.push((ax, ay, bx, by, px, py));
    }
    let mut ans = 0;
    for (ax, ay, bx, by, px, py) in machines {
        let det = ax * by - ay * bx;
        let det_a = px * by - py * bx;
        let det_b = ax * py - ay * px;

        let a = if det_a % det == 0 {
            det_a / det
        } else {
            continue;
        };

        let b = if det_b % det == 0 {
            det_b / det
        } else {
            continue;
        };

        if a < 0 || b < 0 {
            continue;
        }

        ans += (3 * a + b) as usize
    }
    ans
}

fn skip_at(chars: &mut Chars, c: char) {
    while chars.clone().next() != Some(c) {
        chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day13/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 480)
    }
}
