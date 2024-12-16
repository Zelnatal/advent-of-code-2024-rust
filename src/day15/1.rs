use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/day15/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> usize {
    let mut lines = input.lines();
    let mut warehouse = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
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
    
    for (i, row) in warehouse.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if col == 'O' {
                ans += i * 100 + j
            }
        }
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
        'O' => {
            if move_object(warehouse, to, direction) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day15/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 10092)
    }
}
