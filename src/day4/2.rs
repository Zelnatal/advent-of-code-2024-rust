use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day4/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut cross = vec![vec![0; matrix[0].len()]; matrix.len()];
    let xmas = "MAS".chars();
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'M' {
                if find(&matrix, xmas.clone(), i as isize, j as isize, (-1, 1)) {
                    cross[i - 1][j + 1] += 1;
                }

                if find(&matrix, xmas.clone(), i as isize, j as isize, (1, 1)) {
                    cross[i + 1][j + 1] += 1;
                }

                if find(&matrix, xmas.clone(), i as isize, j as isize, (1, -1)) {
                    cross[i + 1][j - 1] += 1;
                }

                if find(&matrix, xmas.clone(), i as isize, j as isize, (-1, -1)) {
                    cross[i - 1][j - 1] += 1;
                }
            }
        }
    }
    cross
        .into_iter()
        .map(|row| row.into_iter().filter(|n| *n == 2).count())
        .sum::<usize>() as i32
}

fn find(
    matrix: &[Vec<char>],
    mut xmas: Chars<'_>,
    i: isize,
    j: isize,
    direction: (isize, isize),
) -> bool {
    let Some(cur) = xmas.next() else {
        return true;
    };
    if i < 0 || i >= matrix.len() as isize || j < 0 || j >= matrix[0].len() as isize {
        return false;
    }
    if cur != matrix[i as usize][j as usize] {
        return false;
    }
    find(matrix, xmas, i + direction.0, j + direction.1, direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day4/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 9)
    }
}
