use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day23/1.txt").unwrap();
    println!("{}", solver(&input))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ComputersSets<'a>([&'a str; 3]);

impl<'a> ComputersSets<'a> {
    fn new(mut elem: [&'a str; 3]) -> Self {
        elem.sort();
        assert!(elem[0] != elem[1] && elem[1] != elem[2]);
        Self(elem)
    }

    fn start_t(&self) -> bool {
        let mut out = false;
        for e in self.0 {
            out |= e.starts_with('t');
        }
        out
    }
}

fn solver(input: &str) -> i64 {
    let input_connection = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect::<Vec<_>>();
    let mut connections = HashMap::new();
    let mut computers = HashSet::new();
    for (a, b) in input_connection {
        connections
            .entry(a)
            .and_modify(|v: &mut Vec<_>| v.push(b))
            .or_insert(vec![b]);
        connections
            .entry(b)
            .and_modify(|v: &mut Vec<_>| v.push(a))
            .or_insert(vec![a]);
        computers.insert(a);
        computers.insert(b);
    }

    let mut computers_sets = HashSet::new();

    for computer1 in computers {
        if let Some(conn) = connections.get(computer1) {
            for computer2 in conn {
                if let Some(conn) = connections.get(computer2) {
                    for computer3 in conn {
                        if connections
                            .get(computer3)
                            .is_some_and(|v| v.contains(&computer1))
                        {
                            computers_sets
                                .insert(ComputersSets::new([computer1, computer2, computer3]));
                        }
                    }
                }
            }
        }
    }

    computers_sets = computers_sets
        .into_iter()
        .filter(|v| v.start_t())
        .collect::<HashSet<_>>();

    computers_sets.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day23/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 7)
    }
}
