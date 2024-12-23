use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day22/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i64 {
    let numbers_secrets = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ans = 0;
    for num in numbers_secrets {
        let mut secret = num;
        for _ in 0..2000 {
            secret = gen_number(secret);
        }
        ans += secret;
    }
    ans
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day22/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 37327623)
    }
}
