use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day15/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let mut lines = input.lines();
    let mut warehouse = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let movements = lines.flat_map(|line| line.chars()).collect::<Vec<_>>();
    let mut pos = 'pos: {
        for (i, row) in warehouse.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == '@' {
                    break 'pos (i as isize, j as isize);
                }
            }
        }
        unreachable!()
    };

    for movement in movements {
        let direction = match movement {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => unreachable!(),
        };
        if move_object(&mut warehouse, pos, direction) {
            pos = (pos.0 + direction.0, pos.1 + direction.1);
        }
    }

    let mut ans = 0;

    let mut i = 0;
    let mut j = 0;
    while i < warehouse.len() {
        while j < warehouse[0].len() {
            if warehouse[i][j] == '[' {
                ans += 100 * i + j;
                j += 1;
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }

    ans
}

fn move_object(
    warehouse: &mut [Vec<char>],
    from: (isize, isize),
    direction: (isize, isize),
) -> bool {
    let to = (from.0 + direction.0, from.1 + direction.1);
    match warehouse[to.0 as usize][to.1 as usize] {
        '#' => false,
        '[' => {
            if possible_move(warehouse, from, direction) {
                if direction == (0, 1) {
                    move_object(warehouse, (to.0, to.1 + 1), direction);
                } else {
                    move_object(warehouse, to, direction);
                    move_object(warehouse, (to.0, to.1 + 1), direction);
                }
                move_box(warehouse, to, (to.0 + direction.0, to.1 + direction.1));

                warehouse[to.0 as usize][to.1 as usize] =
                    warehouse[from.0 as usize][from.1 as usize];
                warehouse[from.0 as usize][from.1 as usize] = '.';

                true
            } else {
                false
            }
        }
        ']' => {
            if possible_move(warehouse, from, direction) {
                if direction == (0, -1) {
                    move_object(warehouse, (to.0, to.1 - 1), direction);
                } else {
                    move_object(warehouse, to, direction);
                    move_object(warehouse, (to.0, to.1 - 1), direction);
                }
                move_box(
                    warehouse,
                    (to.0, to.1 - 1),
                    (to.0 + direction.0, to.1 + direction.1 - 1),
                );

                warehouse[to.0 as usize][to.1 as usize] =
                    warehouse[from.0 as usize][from.1 as usize];
                warehouse[from.0 as usize][from.1 as usize] = '.';

                true
            } else {
                false
            }
        }
        '.' => {
            warehouse[to.0 as usize][to.1 as usize] = warehouse[from.0 as usize][from.1 as usize];
            warehouse[from.0 as usize][from.1 as usize] = '.';
            true
        }
        _ => unreachable!(),
    }
}

fn move_box(warehouse: &mut [Vec<char>], from: (isize, isize), to: (isize, isize)) {
    let pair_to = (to.0, to.1 + 1);

    let pair_from = (from.0, from.1 + 1);

    warehouse[from.0 as usize][from.1 as usize] = '.';
    warehouse[pair_from.0 as usize][pair_from.1 as usize] = '.';
    warehouse[to.0 as usize][to.1 as usize] = '[';
    warehouse[pair_to.0 as usize][pair_to.1 as usize] = ']';
}

fn possible_move(warehouse: &[Vec<char>], from: (isize, isize), direction: (isize, isize)) -> bool {
    let to = (from.0 + direction.0, from.1 + direction.1);
    match warehouse[to.0 as usize][to.1 as usize] {
        '#' => false,
        '[' => {
            if direction == (0, 1) {
                possible_move(warehouse, (to.0, to.1 + 1), direction)
            } else if direction == (0, -1) {
                possible_move(warehouse, to, direction)
            } else {
                possible_move(warehouse, to, direction)
                    && possible_move(warehouse, (to.0, to.1 + 1), direction)
            }
        }
        ']' => possible_move(warehouse, (from.0, from.1 - 1), direction),
        '.' => true,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day15/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 9021)
    }
}
