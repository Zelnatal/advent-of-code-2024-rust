use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day16/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let maze = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_pos = 'start_pos: {
        for (i, row) in maze.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == 'S' {
                    break 'start_pos (i as isize, j as isize, 0, 1);
                }
            }
        }
        unreachable!()
    };

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_pos)));

    let mut seen = HashSet::new();

    while let Some(Reverse((score, cur_pos))) = queue.pop() {
        if maze[cur_pos.0 as usize][cur_pos.1 as usize] == 'E' {
            return score;
        }
        if seen.insert((cur_pos.0, cur_pos.1)) {
            let next_pos = (cur_pos.0 + cur_pos.2, cur_pos.1 + cur_pos.3);
            if maze[next_pos.0 as usize][next_pos.1 as usize] != '#' {
                queue.push(Reverse((
                    score + 1,
                    (next_pos.0, next_pos.1, cur_pos.2, cur_pos.3),
                )));
            }

            let clockwise = match (cur_pos.2, cur_pos.3) {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };

            let next_pos = (cur_pos.0 + clockwise.0, cur_pos.1 + clockwise.1);
            if maze[next_pos.0 as usize][next_pos.1 as usize] != '#' {
                queue.push(Reverse((
                    score + 1001,
                    (next_pos.0, next_pos.1, clockwise.0, clockwise.1),
                )));
            }

            let counterclockwise = match (cur_pos.2, cur_pos.3) {
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                _ => unreachable!(),
            };

            let next_pos = (
                cur_pos.0 + counterclockwise.0,
                cur_pos.1 + counterclockwise.1,
            );
            if maze[next_pos.0 as usize][next_pos.1 as usize] != '#' {
                queue.push(Reverse((
                    score + 1001,
                    (
                        next_pos.0,
                        next_pos.1,
                        counterclockwise.0,
                        counterclockwise.1,
                    ),
                )));
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day16/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 7036)
    }
}
