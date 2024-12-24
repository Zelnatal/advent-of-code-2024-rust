use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day24/2.txt").unwrap();
    println!("{}", solver(&input))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl From<&str> for GateKind {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    o1: String,
    o2: String,
    op: GateKind,
    res: String,
}

impl Gate {
    fn new(mut o1: String, mut o2: String, op: GateKind, res: String) -> Self {
        if o1 > o2 {
            std::mem::swap(&mut o1, &mut o2);
        }
        Self { o1, o2, op, res }
    }
}

fn solver(input: &str) -> String {
    let mut lines = input.lines();
    let _initial_value = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut chars = line.chars();
            let key = chars.by_ref().take_while(|c| *c != ':').collect::<String>();
            chars.next();
            let value = chars.next().unwrap() == '1';
            (key, value)
        })
        .collect::<HashMap<_, _>>();

    let mut z_max = 0;
    let gates = lines
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
            if parts[4].starts_with('z') {
                z_max = z_max.max(parts[4][1..].parse::<i32>().unwrap())
            }
            Gate::new(
                parts[0].to_owned(),
                parts[2].to_owned(),
                parts[1].into(),
                parts[4].to_owned(),
            )
        })
        .collect::<Vec<_>>();

    let mut ans: Vec<&str> = Vec::new();

    for gate in gates.iter() {
        if gate.op == GateKind::Xor
            && gate.o1.starts_with('x')
            && gate.o1 != "x00"
            && gate.o2.starts_with('y')
            && gate.o2 != "y00"
            && gate.res.starts_with('z')
        {
            ans.push(&gate.res);
        }

        if gate.op == GateKind::And && gate.res.starts_with('z') {
            ans.push(&gate.res);
        }

        if gate.op == GateKind::Or
            && gate.res.starts_with('z')
            && gate.res != format!("z{:2}", z_max)
        {
            ans.push(&gate.res);
        }

        if gate.op == GateKind::Xor
            && gate.o1.starts_with('x')
            && gate.o2.starts_with('y')
            && gates.iter().any(|or_gate| {
                or_gate.op == GateKind::Or && or_gate.o1 == gate.res || or_gate.o2 == or_gate.res
            })
        {
            ans.push(&gate.res);
        }

        if gate.op == GateKind::Xor
            && !(gate.o1.starts_with('x') && gate.o2.starts_with('y') || gate.res.starts_with('z'))
        {
            ans.push(&gate.res);
        }

        if gate.op == GateKind::And
            && gate.o1 != "x00"
            && gate.o2 != "y00"
            && gates.iter().any(|not_or_gate| {
                not_or_gate.op != GateKind::Or && not_or_gate.o1 == gate.res
                    || not_or_gate.o2 == not_or_gate.res
            })
        {
            ans.push(&gate.res);
        }
    }

    ans.sort();
    ans.join(",")
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exemple() {
//         let input = read_to_string("./input/day24/2_exemple.txt").unwrap();
//         assert_eq!(solver(&input), "z00,z01,z02,z05".to_string())
//     }
// }
