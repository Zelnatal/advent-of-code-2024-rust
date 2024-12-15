use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day12/2.txt").unwrap();
    println!("{}", solver(&input))
}

const DIRECTION: [isize; 5] = [-1, 0, 1, 0, -1];

fn solver(input: &str) -> usize {
    let farm = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut seen = HashSet::new();
    let mut ans = 0;
    for i in 0..farm.len() as isize {
        for j in 0..farm[0].len() as isize {
            if seen.insert((i, j)) {
                let (area, perimeter) = step(&farm, i, j, 0, 0, &mut seen, &mut HashSet::new());
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
    fence: &mut HashSet<(isize, isize, usize)>,
) -> (usize, usize) {
    let cur = farm[i as usize][j as usize];
    area += 1;

    let mut steps = Vec::new();
    for d in 0..4 {
        let ni = i + DIRECTION[d];
        let nj = j + DIRECTION[d + 1];
        if (0..farm.len() as isize).contains(&ni)
            && (0..farm[0].len() as isize).contains(&nj)
            && farm[ni as usize][nj as usize] == cur
        {
            if seen.insert((ni, nj)) {
                steps.push((ni, nj));
            }
        } else if fence.insert((ni, nj, d)) {
            perimeter += 1;
            set_fence(farm, fence, i, j, d, cur);
        }
    }

    for (ni, nj) in steps {
        (area, perimeter) = step(farm, ni, nj, area, perimeter, seen, fence)
    }
    (area, perimeter)
}

fn set_fence(
    farm: &[Vec<char>],
    fence: &mut HashSet<(isize, isize, usize)>,
    i: isize,
    j: isize,
    d: usize,
    cur: char,
) {
    let d1 = (d + 3) % 4;
    let mut ni = i + DIRECTION[d1];
    let mut nj = j + DIRECTION[d1 + 1];
    while (0..farm.len() as isize).contains(&ni)
        && (0..farm[0].len() as isize).contains(&nj)
        && farm[ni as usize][nj as usize] == cur
    {
        let fi = ni + DIRECTION[d];
        let fj = nj + DIRECTION[d + 1];
        if (0..farm.len() as isize).contains(&fi)
            && (0..farm[0].len() as isize).contains(&fj)
            && farm[fi as usize][fj as usize] == cur
        {
            break;
        }
        fence.insert((fi, fj, d));
        ni += DIRECTION[d1];
        nj += DIRECTION[d1 + 1];
    }

    let d2 = (d + 1) % 4;
    ni = i + DIRECTION[d2];
    nj = j + DIRECTION[d2 + 1];
    while (0..farm.len() as isize).contains(&ni)
        && (0..farm[0].len() as isize).contains(&nj)
        && farm[ni as usize][nj as usize] == cur
    {
        let fi = ni + DIRECTION[d];
        let fj = nj + DIRECTION[d + 1];
        if (0..farm.len() as isize).contains(&fi)
            && (0..farm[0].len() as isize).contains(&fj)
            && farm[fi as usize][fj as usize] == cur
        {
            break;
        }
        fence.insert((fi, fj, d));
        ni += DIRECTION[d2];
        nj += DIRECTION[d2 + 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day12/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 1206)
    }
}
