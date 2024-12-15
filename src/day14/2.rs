use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day14/2.txt").unwrap();
    println!("{}", solver(&input))
}

// #[cfg(test)]
// const WIDE: isize = 11;
// #[cfg(not(test))]
const WIDE: isize = 101;
// #[cfg(test)]
// const TALL: isize = 7;
// #[cfg(not(test))]
const TALL: isize = 103;

fn solver(input: &str) -> usize {
    println!("{} {}", WIDE, TALL);
    let mut robots = Vec::new();
    let mut vel = Vec::new();
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

        robots.push((px, py));
        vel.push((vx, vy));
    }
    for i in 0..10_000{
        step(&mut robots, &vel);
        let s = print(&robots);
        if possible_tree(&s) {
            println!("{}", i + 1);
            println!("{}", s)
        }
    }

    0
}

fn skip_at(chars: &mut Chars, c: char) {
    while chars.clone().next() != Some(c) {
        chars.next();
    }
}

fn step(robots: &mut [(isize, isize)], vel: &[(isize, isize)]) {
    for (i, robot) in robots.iter_mut().enumerate() {
        robot.0 = (robot.0 + vel[i].0).rem_euclid(WIDE);
        robot.1 = (robot.1 + vel[i].1).rem_euclid(TALL);
    }
}

fn print(robots: &[(isize, isize)]) -> String {
    let mut out = String::new();
    for j in 0..TALL {
        for i in 0..WIDE {
            if robots.iter().any(|x| *x == (i, j)) {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        out.push('\n');
    }
    out
}

fn possible_tree(s: &str) -> bool {
    for line in s.lines() {
        if line.contains(&"#".repeat(10)) {
            return true;
        }
    }
    false
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exemple() {
//         let input = read_to_string("./input/day14/2_exemple.txt").unwrap();
//         assert_eq!(solver(&input), 1206)
//     }
// }
