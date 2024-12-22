use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day17/1.txt").unwrap();
    println!("{}", solver(&input))
}

macro_rules! combo {
    ($combo:expr, $register_a:expr, $register_b:expr, $register_c:expr) => {
        match $combo {
            n @ 0..=3 => n,
            4 => $register_a,
            5 => $register_b,
            6 => $register_c,
            _ => unreachable!(),
        }
    };
}

fn solver(input: &str) -> String {
    let mut lines = input.lines();

    let mut register_a = lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let mut register_b = lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    let mut register_c = lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    lines.next();

    let program = lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .collect::<String>()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut out = Vec::new();

    let mut i = 0;
    while i < program.len() - 1 {
        match program[i] {
            0 => {
                let num = register_a;
                let div = 2_i32.pow(combo!(program[i + 1], register_a, register_b, register_c) as u32);
                register_a = num / div;
            }
            1 => register_b ^= program[i + 1],
            2 => register_b = combo!(program[i + 1], register_a, register_b, register_c) % 8,
            3 if register_a != 0 => {
                i = program[i + 1] as usize;
                continue;
            }
            4 => register_b ^= register_c,
            5 => out.push(combo!(program[i + 1], register_a, register_b, register_c) % 8),
            6 => {
                let num = register_a;
                let div = 2_i32.pow(combo!(program[i + 1], register_a, register_b, register_c) as u32);
                register_b = num / div;
            }
            7 => {
                let num = register_a;
                let div = 2_i32.pow(combo!(program[i + 1], register_a, register_b, register_c) as u32);
                register_c = num / div;
            }
            _ => {}
        }
        i += 2;
    }

    out.into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day17/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), "4,6,3,5,6,3,5,2,1,0".to_string())
    }
}
