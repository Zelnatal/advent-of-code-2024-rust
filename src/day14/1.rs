use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day14/1.txt").unwrap();
    println!("{}", solver(&input))
}

#[cfg(test)]
const WIDE: isize = 11;
#[cfg(not(test))]
const WIDE: isize = 101;
#[cfg(test)]
const TALL: isize = 7;
#[cfg(not(test))]
const TALL: isize = 103;

fn solver(input: &str) -> usize {
    println!("{} {}",WIDE,TALL);
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();

        skip_at(&mut chars, '=');
        chars.next();
        let px = chars
            .clone()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        skip_at(&mut chars, ',');
        chars.next();
        let py = chars
            .clone()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        skip_at(&mut chars, '=');
        chars.next();
        let vx = chars
            .clone()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        skip_at(&mut chars, ',');
        chars.next();
        let vy = chars
            .clone()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        let npx = (px + vx * 100).rem_euclid(WIDE);
        let npy = (py + vy * 100).rem_euclid(TALL);

        robots.push((npx, npy));
    }
    let mut quadrant = (0, 0, 0, 0);
    for (px, py) in robots {
        if px < WIDE / 2 && py < TALL / 2 {
            quadrant.0 += 1
        } else if px > WIDE / 2 && py < TALL / 2 {
            quadrant.1 += 1
        } else if px < WIDE / 2 && py > TALL / 2 {
            quadrant.2 += 1
        } else if px > WIDE / 2 && py > TALL / 2 {
            quadrant.3 += 1;
        }
    }
    quadrant.0 * quadrant.1 * quadrant.2 * quadrant.3
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
        let input = read_to_string("./input/day14/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 12)
    }
}
