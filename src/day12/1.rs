use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day12/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let farm = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut seen = HashSet::new();
    let mut ans = 0;
    for i in 0..farm.len() as isize {
        for j in 0..farm[0].len() as isize {
            if !seen.contains(&(i, j)) {
                let (area, perimeter) = step(&farm, i, j, 0, 0, &mut seen);
                ans += area * perimeter;
            }
        }
    }
    ans
}

fn step(
    farm: &[Vec<char>],
    i: isize,
    j: isize,
    mut area: usize,
    mut perimeter: usize,
    seen: &mut HashSet<(isize, isize)>,
) -> (usize, usize) {
    let cur = farm[i as usize][j as usize];
    seen.insert((i, j));
    area += 1;
    const DIRECTION: [isize; 5] = [-1, 0, 1, 0, -1];
    for d in 0..4 {
        let ni = i + DIRECTION[d];
        let nj = j + DIRECTION[d + 1];
        if (0..farm.len() as isize).contains(&ni)
            && (0..farm[0].len() as isize).contains(&nj)
            && farm[ni as usize][nj as usize] == cur
        {
            if !seen.contains(&(ni, nj)) {
                (area, perimeter) = step(farm, ni, nj, area, perimeter, seen)
            }
        } else {
            perimeter += 1;
        }
    }
    (area, perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day12/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 1930)
    }
}
