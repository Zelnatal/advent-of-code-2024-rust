use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day6/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut guard = (0, 0);
    'found_guard: for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                guard = (i as isize, j as isize);
                break 'found_guard;
            }
        }
    }
    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                map[i][j] = '#';
                if detect_loop(&mut map, guard) {
                    ans += 1
                }
                map[guard.0 as usize][guard.1 as usize] = '^';
                map[i][j] = '.';
            }
        }
    }
    ans
}

fn detect_loop(map: &mut [Vec<char>], mut guard: (isize, isize)) -> bool {
    let mut possible_loop = HashSet::new();
    loop {
        let guard_char = map[guard.0 as usize][guard.1 as usize];
        let direction = match guard_char {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => unreachable!(),
        };
        map[guard.0 as usize][guard.1 as usize] = '.';
        let new_pos = (guard.0 + direction.0, guard.1 + direction.1);
        if new_pos.0 < 0
            || new_pos.0 >= map.len() as isize
            || new_pos.1 < 0
            || new_pos.1 >= map[0].len() as isize
        {
            break false;
        }

        if possible_loop.contains(&(guard, guard_char)) {
            break true;
        }

        if map[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            map[guard.0 as usize][guard.1 as usize] = match guard_char {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => unreachable!(),
            };
        } else {
            possible_loop.insert((guard, guard_char));
            map[new_pos.0 as usize][new_pos.1 as usize] = guard_char;
            guard = new_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day6/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 6)
    }
}
