use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day3/2.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut cursor = Cursor {
        chars: input.chars(),
    };
    let mut ans = 0;
    let mut enable = true;
    while !cursor.is_at_end() {
        if cursor.get_string(3).is_some_and(|s| s.as_str() == "mul") && enable {
            cursor.next_char();
            cursor.next_char();
            cursor.next_char();

            if cursor.first() != '(' {
                continue;
            }
            cursor.next_char();

            if !cursor.first().is_ascii_digit() {
                continue;
            }
            let mut num1 = cursor.next_char().to_string();
            while cursor.first().is_ascii_digit() {
                num1.push(cursor.next_char());
            }

            if cursor.first() != ',' {
                continue;
            }
            cursor.next_char();

            if !cursor.first().is_ascii_digit() {
                continue;
            }
            let mut num2 = cursor.next_char().to_string();
            while cursor.first().is_ascii_digit() {
                num2.push(cursor.next_char());
            }

            if cursor.first() != ')' {
                continue;
            }
            cursor.next_char();

            ans += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
        } else if cursor.get_string(4).is_some_and(|s| s.as_str() == "do()") {
            for _ in 0..3 {
                cursor.next_char();
            }
            enable = true
        } else if cursor
            .get_string(7)
            .is_some_and(|s| s.as_str() == "don't()")
        {
            for _ in 0..7 {
                cursor.next_char();
            }
            enable = false
        } else {
            cursor.next_char();
        }
    }
    ans
}

struct Cursor<'a> {
    chars: Chars<'a>,
}

impl Cursor<'_> {
    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn get_string(&self, len: usize) -> Option<String> {
        let mut string = String::with_capacity(len);
        let mut chars = self.chars.clone();
        for _ in 0..len {
            string.push(chars.next()?);
        }
        Some(string)
    }

    fn next_char(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.chars.clone().next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple() {
        let input = read_to_string("./input/day3/2_exemple.txt").unwrap();
        assert_eq!(solver(&input), 48)
    }
}
