use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day18/2.txt").unwrap();
    println!("{}", solver(&input,70))
}

fn solver(input: &str, len: usize) -> String {
    let positions = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let mut ans = String::new();
    let mut left = 0;
    let mut right = positions.len()-1;
    while left <= right {
        let mid = (left + right)/2;
        let mut memory = vec![vec![false; len + 1]; len + 1];
        for &(x,y) in positions.iter().take(mid+1) {
            memory[x][y] = true
        }
        if possible(&memory, len) {
            left = mid + 1;
        } else {
            ans = format!("{},{}",positions[mid].0,positions[mid].1);
            right = mid -1;
        }
    }

    ans
}

fn possible(memory: &[Vec<bool>], len: usize) -> bool {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Reverse((0, 0, 0)));

    while let Some(Reverse((s, x, y))) = queue.pop() {
        if visited.insert((x, y)) {
            if x == len && y == len {
                return true;
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
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day18/2_exemple.txt").unwrap();
        assert_eq!(solver(&input,6), "6,1".to_string())
    }
}
