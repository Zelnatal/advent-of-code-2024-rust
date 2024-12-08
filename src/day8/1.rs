use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day8/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut antennas = HashMap::new();
    let mut antennas_pos = Vec::new();
    let mut antinodes = HashSet::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if matches!(col,'0'..='9' | 'a'..='z' | 'A'..='Z') {
                antennas
                    .entry(col)
                    .and_modify(|v: &mut Vec<(isize, isize)>| v.push((i as isize, j as isize)))
                    .or_insert(vec![(i as isize, j as isize)]);
                antennas_pos.push((col, (i as isize, j as isize)));
            }
        }
    }
    for (antenna, (cur_i, cur_j)) in antennas_pos {
        for &(other_i, other_j) in antennas.get(&antenna).unwrap() {
            let possible_i = (other_i - cur_i) + other_i;
            let possible_j = (other_j - cur_j) + other_j;
            if cur_i == other_i
                || cur_j == other_j
                || possible_i < 0
                || possible_i >= map.len() as isize
                || possible_j < 0
                || possible_j >= map[0].len() as isize
            {
                continue;
            }
            antinodes.insert((possible_i, possible_j));
        }
    }
    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day8/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 14)
    }
}
