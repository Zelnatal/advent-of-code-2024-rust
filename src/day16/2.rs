use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day16/2.txt").unwrap();
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

    let best_score = 'best_score: {
        while let Some(Reverse((score, cur_pos))) = queue.pop() {
            if maze[cur_pos.0 as usize][cur_pos.1 as usize] == 'E' {
                break 'best_score score;
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
    };

    let mut possible_tile = HashSet::new();

    possible_tile.insert((start_pos.0, start_pos.1));

    step(
        &maze,
        (start_pos.0, start_pos.1),
        (start_pos.2, start_pos.3),
        0,
        best_score,
        &mut Vec::new(),
        &mut possible_tile,
        &mut HashMap::new(),
    );

    possible_tile.len()
}

#[allow(clippy::too_many_arguments)]
fn step(
    maze: &[Vec<char>],
    cur_pos: (isize, isize),
    direction: (isize, isize),
    cur_score: i32,
    best_score: i32,
    cur_path: &mut Vec<(isize, isize)>,
    possible_tile: &mut HashSet<(isize, isize)>,
    cache: &mut HashMap<(isize, isize, isize, isize), (i32, bool)>,
) -> bool {
    if cur_score > best_score {
        return false;
    }

    if let Some(score) = cache.get(&(cur_pos.0, cur_pos.1, direction.0, direction.1)) {
        if score.0 == cur_score && score.1 {
            for path in cur_path {
                possible_tile.insert(*path);
            }
            return true;
        } else if cur_score >= score.0 {
            return false;
        }
    }

    if maze[cur_pos.0 as usize][cur_pos.1 as usize] == 'E' {
        for path in cur_path {
            possible_tile.insert(*path);
        }
        return true;
    }

    let mut out = false;

    let next_pos = (cur_pos.0 + direction.0, cur_pos.1 + direction.1);
    if maze[next_pos.0 as usize][next_pos.1 as usize] != '#' {
        cur_path.push(next_pos);
        out |= step(
            maze,
            next_pos,
            direction,
            cur_score + 1,
            best_score,
            cur_path,
            possible_tile,
            cache,
        );
        cur_path.pop();
    }

    let clockwise = match direction {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    };

    let next_pos = (cur_pos.0 + clockwise.0, cur_pos.1 + clockwise.1);
    if maze[next_pos.0 as usize][next_pos.1 as usize] != '#' {
        cur_path.push(next_pos);
        out |= step(
            maze,
            next_pos,
            clockwise,
            cur_score + 1001,
            best_score,
            cur_path,
            possible_tile,
            cache,
        );
        cur_path.pop();
    }

    let counterclockwise = match direction {
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
        cur_path.push(next_pos);
        out |= step(
            maze,
            next_pos,
            counterclockwise,
            cur_score + 1001,
            best_score,
            cur_path,
            possible_tile,
            cache,
        );
        cur_path.pop();
    }

    cache.insert(
        (cur_pos.0, cur_pos.1, direction.0, direction.1),
        (cur_score, out),
    );

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day16/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 45)
    }
}
