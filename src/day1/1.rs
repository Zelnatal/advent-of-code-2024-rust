use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day1/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for line in input.lines() {
        let nums = line.split("   ").collect::<Vec<_>>();
        vec1.push(nums[0].parse::<i32>().unwrap());
        vec2.push(nums[1].parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    vec1.into_iter()
        .zip(vec2)
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day1/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 11)
    }
}
