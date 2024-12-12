use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day11/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let stones = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut ans = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        ans += blink(stone, 0, &mut cache);
    }
    ans
}

fn blink(stone: usize, count: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if count == 75 {
        return 1;
    }
    if let Some(r) = cache.get(&(stone, count)) {
        return *r;
    }
    let stones = match (stone, digits(stone)) {
        (0, _) => blink(1, count + 1, cache),
        (_, d) if d % 2 == 0 => {
            let (n1, n2) = split_number(stone, d);
            blink(n1, count + 1, cache) + blink(n2, count + 1, cache)
        }
        (_, _) => blink(stone * 2024, count + 1, cache),
    };
    cache.insert((stone, count), stones);
    stones
}

fn digits(mut n: usize) -> u32 {
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

fn split_number(n: usize, digits: u32) -> (usize, usize) {
    let div = 10_usize.pow(digits / 2);
    (n / div, n % div)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exemple() {
//         let input = read_to_string("./input/day11/2_exemple.txt").unwrap();
//         assert_eq!(solver(&input), 81)
//     }
// }
