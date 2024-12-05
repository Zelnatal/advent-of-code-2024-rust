use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day4/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut ans = 0;
    let xmas = "XMAS".chars();
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'X' {
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (-1, 0)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (-1, 1)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (0, 1)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (1, 1)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (1, 0)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (1, -1)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (0, -1)));
                ans += i32::from(find(&matrix, xmas.clone(), i as isize, j as isize, (-1, -1)));

            }
        }
    }
    ans
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
        let input = read_to_string("./input/day4/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 18)
    }
}
