use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day18/1.txt").unwrap();
    println!("{}", solver(&input, 70, 1024))
}

fn solver(input: &str, len: usize, bytes: usize) -> i32 {
    let positions = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let mut memory = vec![vec![false; len + 1]; len + 1];

    for (x, y) in positions.into_iter().take(bytes) {
        memory[x][y] = true;
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Reverse((0, 0, 0)));

    while let Some(Reverse((s, x, y))) = queue.pop() {
        if visited.insert((x, y)) {
            if x == len && y == len {
                return s;
            }
            const DIRECTION: [isize; 5] = [-1, 0, 1, 0, -1];
            for d in 0..4 {
                let nx = x as isize + DIRECTION[d];
                let ny = y as isize + DIRECTION[d + 1];
                if (0..=len as isize).contains(&nx)
                    && (0..=len as isize).contains(&ny)
                    && !memory[nx as usize][ny as usize]
                {
                    queue.push(Reverse((s + 1, nx as usize, ny as usize)));
                }
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day18/1_exemple.txt").unwrap();
        assert_eq!(solver(&input, 6, 12), 22)
    }
}
