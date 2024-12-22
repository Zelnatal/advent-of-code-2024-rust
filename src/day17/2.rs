use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day17/2.txt").unwrap();
    println!("{}", solver(&input))
}

macro_rules! combo {
    ($combo:expr, $a:expr, $b:expr, $c:expr) => {
        match $combo {
            n @ 0..=3 => n,
            4 => $a,
            5 => $b,
            6 => $c,
            _ => unreachable!(),
        }
    };
}

fn solver(input: &str) -> i64 {
    let mut lines = input.lines();
    lines.next();
    lines.next();
    lines.next();
    lines.next();

    let program = lines
        .next()
        .unwrap()
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .collect::<String>()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ans = 0;
    for &n in program.iter().rev() {
        let o = 'o: {
            for s_a in 0..=7 {
                let mut a = (ans << 3) | s_a;
                let mut b = 0;
                let mut c = 0;
                let mut i = 0;
                while i < program.len() - 2 {
                    match program[i] {
                        0 => {
                            let num = a;
                            let div = 2_i64.pow(combo!(program[i + 1], a, b, c) as u32);
                            a = num / div;
                        }
                        1 => b ^= program[i + 1],
                        2 => b = combo!(program[i + 1], a, b, c) % 8,
                        3 if a != 0 => {
                            i = program[i + 1] as usize;
                            continue;
                        }
                        4 => b ^= c,
                        5 => {
                            if combo!(program[i + 1], a, b, c) % 8 == n {
                                break 'o s_a;
                            }
                        }
                        6 => {
                            let num = a;
                            let div = 2_i64.pow(combo!(program[i + 1], a, b, c) as u32);
                            b = num / div;
                        }
                        7 => {
                            let num = a;
                            let div = 2_i64.pow(combo!(program[i + 1], a, b, c) as u32);
                            c = num / div;
                        }
                        _ => {}
                    }
                    i += 2;
                }
            }
            unreachable!()
        };
        ans <<= 3;
        ans |= o;
    }
    ans
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exemple() {
//         let input = read_to_string("./input/day17/2_exemple.txt").unwrap();
//         assert_eq!(solver(&input), 117440)
//     }
// }
