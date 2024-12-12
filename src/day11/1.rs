use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day11/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let mut stones = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            match stones[i] {
                0 => stones[i] = 1,
                n if n.to_string().len() % 2 == 0 => {
                    let (n1, n2) = {
                        let tmp = n.to_string();
                        (
                            tmp[..tmp.len() / 2].parse::<usize>().unwrap(),
                            tmp[tmp.len() / 2..].parse::<usize>().unwrap(),
                        )
                    };
                    stones.remove(i);
                    stones.insert(i, n2);
                    stones.insert(i, n1);
                    i += 1;
                }
                _ => stones[i] *= 2024,
            }
            i += 1;
        }
    }
    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day11/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 55312)
    }
}
