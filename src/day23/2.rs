use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("./input/day23/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> String {
    let input_connection = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect::<Vec<_>>();
    let mut connections = HashMap::new();
    let mut computers = HashSet::new();
    let mut in_degree = HashMap::new();
    for (a, b) in input_connection {
        connections
            .entry(a)
            .and_modify(|v: &mut HashSet<_>| {
                v.insert(b);
            })
            .or_insert(HashSet::from_iter(vec![b].into_iter()));
        connections
            .entry(b)
            .and_modify(|v: &mut HashSet<_>| {
                v.insert(a);
            })
            .or_insert(HashSet::from_iter(vec![a].into_iter()));
        computers.insert(a);
        computers.insert(b);
        in_degree.entry(a).and_modify(|d| *d += 1).or_insert(1);
        in_degree.entry(b).and_modify(|d| *d += 1).or_insert(1);
    }

    let mut output = Vec::new();

    bron_kerbosch(
        HashSet::new(),
        computers,
        HashSet::new(),
        &connections,
        &mut output,
    );
    output.sort_by_key(|s| s.len());
    let largest = output.last().unwrap().clone();
    let mut password = largest.into_iter().collect::<Vec<_>>();
    password.sort();
    password.join(",")
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    output: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        output.push(r);
        return;
    }
    let pivot_u = *p.union(&x).next().unwrap();
    let vertex = p
        .difference(connections.get(pivot_u).unwrap())
        .copied()
        .collect::<HashSet<_>>();
    for vertex in vertex {
        let mut new_r = r.clone();
        new_r.insert(vertex);
        let new_p = p
            .intersection(connections.get(vertex).unwrap())
            .copied()
            .collect::<HashSet<_>>();
        let new_x = x
            .intersection(connections.get(vertex).unwrap())
            .copied()
            .collect::<HashSet<_>>();
        bron_kerbosch(new_r, new_p, new_x, connections, output);
        p.remove(vertex);
        x.insert(vertex);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day23/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), "co,de,ka,ta".to_string())
    }
}
