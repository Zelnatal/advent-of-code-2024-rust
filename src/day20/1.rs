use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day20/1.txt").unwrap();
    println!("{}", solver(&input, 100))
}

fn solver(input: &str, min: i32) -> i32 {
    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => -1_i32,
                    '.' => 0,
                    'S' => -2,
                    'E' => 0,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let pos_s = 'pos_s: {
        for (i, row) in map.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col == -2 {
                    break 'pos_s (i, j);
                }
            }
        }
        unreachable!()
    };

    let mut cur = pos_s;
    let mut time = 1;
    loop {
        map[cur.0][cur.1] = time;
        time += 1;
        let mut repeat = false;
        const DIRECTION: [isize; 5] = [-1, 0, 1, 0, -1];
        for d in 0..4 {
            let ni = (cur.0 as isize + DIRECTION[d]) as usize;
            let nj = (cur.1 as isize + DIRECTION[d + 1]) as usize;
            if map[ni][nj] == 0 {
                cur = (ni, nj);
                repeat = true;
                break;
            }
        }
        if !repeat {
            break;
        }
    }

    let mut cheats = HashMap::new();

    for (i, row) in map.iter().enumerate().skip(1).take(map.len() - 2) {
        for (j, &col) in row.iter().enumerate().skip(1).take(map[0].len() - 2) {
            if col == -1 {
                if map[i - 1][j] != -1 && map[i + 1][j] != -1 {
                    let saving = (map[i - 1][j] - map[i + 1][j]).abs() - 2;
                    cheats.entry(saving).and_modify(|q| *q += 1).or_insert(1);
                }
                if map[i][j - 1] != -1 && map[i][j + 1] != -1 {
                    let saving = (map[i][j - 1] - map[i][j + 1]).abs() - 2;
                    cheats.entry(saving).and_modify(|q| *q += 1).or_insert(1);
                }
            }
        }
    }


    
    let mut ans = 0;
    for (save, cheat) in cheats {
        if save >= min {
            ans += cheat;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day20/1_exemple.txt").unwrap();
        assert_eq!(solver(&input, 20), 5)
    }
}
