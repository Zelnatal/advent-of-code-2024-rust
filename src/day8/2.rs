use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day8/2.txt").unwrap();
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
            if cur_i == other_i || cur_j == other_j {
                continue;
            }
            let direction_i = other_i - cur_i;
            let direction_j = other_j - cur_j;
            let mut possible_i = other_i;
            let mut possible_j = other_j;
            while (0..map.len() as isize).contains(&possible_i)
                && (0..map[0].len() as isize).contains(&possible_j)
            {
                antinodes.insert((possible_i, possible_j));
                possible_i += direction_i;
                possible_j += direction_j;
            }
        }
    }
    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day8/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 34)
    }
}
