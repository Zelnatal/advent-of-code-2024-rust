use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("./input/day24/1.txt").unwrap();
    println!("{}", solver(&input))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

impl From<String> for Gate {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

fn solver(input: &str) -> i64 {
    let mut lines = input.lines();
    let initial_value = lines
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

    let mut gates = lines
        .map(|line| {
            let mut chars = line.chars();

            let o1 = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();

            let gate: Gate = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>()
                .into();

            let o2 = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();

            chars.next();
            chars.next();
            chars.next();
            let result = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();
            (o1, o2, gate, result)
        })
        .collect::<Vec<_>>();
    
    let mut variables = initial_value.clone();
    let mut i = 0;
    while !gates.is_empty() {
        i %= gates.len();
        let gate = &gates[i];
        if let (Some(&o1), Some(&o2)) = (variables.get(&gate.0), variables.get(&gate.1)) {
            let result = match gate.2 {
                Gate::And => o1 & o2,
                Gate::Or => o1 | o2,
                Gate::Xor => o1 ^ o2,
            };
            variables.insert(gates.remove(i).3, result);
        } else {
            i += 1;
        }
    }
    let mut bits = variables
        .into_iter()
        .filter(|(keys, _)| keys.starts_with('z'))
        .collect::<Vec<_>>();
    bits.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    for (_, v) in bits {
        ans <<= 1;
        ans |= v as i64;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day24/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 2024)
    }
}
