use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day1/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut freq = HashMap::new();
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for line in input.lines() {
        let nums = line.split("   ").collect::<Vec<_>>();
        freq.insert(nums[0].parse::<i32>().unwrap(), 0);
        vec1.push(nums[0].parse::<i32>().unwrap());
        vec2.push(nums[1].parse::<i32>().unwrap());
    }
    for n in vec2 {
        if let Some(freq) = freq.get_mut(&n) {
            *freq += 1;
        }
    }
    vec1.into_iter().map(|n| n * freq.get(&n).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day1/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 31)
    }
}
