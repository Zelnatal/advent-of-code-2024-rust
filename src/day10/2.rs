use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day10/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let topo_map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map_or(-1, |n| n as i32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (i, row) in topo_map.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == 0 {
                ans += step(&topo_map, i as isize, j as isize);
            }
        }
    }
    ans
}

fn step(topo_map: &[Vec<i32>], i: isize, j: isize) -> usize {
    if topo_map[i as usize][j as usize] == 9 {
        return 1;
    }
    let mut out = 0;
    const DIRECTION: [isize; 5] = [-1, 0, 1, 0, -1];
    for d in 0..4 {
        let ni = i + DIRECTION[d];
        let nj = j + DIRECTION[d + 1];
        if (0..topo_map.len() as isize).contains(&ni)
            && (0..topo_map[0].len() as isize).contains(&nj)
            && topo_map[i as usize][j as usize] + 1 == topo_map[ni as usize][nj as usize]
        {
            out += step(topo_map, ni, nj)
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day10/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 81)
    }
}
