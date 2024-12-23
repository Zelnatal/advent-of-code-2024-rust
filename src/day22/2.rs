use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day22/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i64 {
    let numbers_secrets = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut combination = HashMap::new();
    for num in numbers_secrets {
        let mut secret = num;
        let mut prices = Vec::with_capacity(2000);
        for _ in 0..2000 {
            prices.push(get_ones(secret));
            secret = gen_number(secret);
        }
        let mut changes = vec![0; 2000];
        for i in 1..2000 {
            changes[i] = prices[i] - prices[i - 1]
        }
        let mut seen = HashSet::new();
        for i in 4..2000 {
            let comb = (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
            if seen.insert(comb) {
                combination
                    .entry(comb)
                    .and_modify(|p| *p += prices[i])
                    .or_insert(prices[i]);
            }
        }
    }

    combination.into_values().max().unwrap()
}

macro_rules! mix_prune {
    ($secret:ident,$result:expr) => {
        $secret ^= $result;
        $secret %= 16777216;
    };
}

fn gen_number(mut secret: i64) -> i64 {
    let mut result = secret * 64;
    mix_prune!(secret, result);
    result = secret / 32;
    mix_prune!(secret, result);
    result = secret * 2048;
    mix_prune!(secret, result);
    secret
}

fn get_ones(number: i64) -> i64 {
    number % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day22/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 23)
    }
}
