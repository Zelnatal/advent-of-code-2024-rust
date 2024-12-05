use std::{fs::read_to_string, str::Chars};

fn main() {
    let input = read_to_string("./input/day3/1.txt").unwrap();
    println!("{}", solver(&input))
}

fn solver(input: &str) -> i32 {
    let mut cursor = Cursor {
        chars: input.chars(),
    };
    let mut ans = 0;
    while !cursor.is_at_end() {
        if cursor.first() == 'm' && cursor.second() == 'u' && cursor.third() == 'l' {
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
        } else {
            cursor.next_char();
            cursor.skip_at_m();
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

    fn second(&self) -> char {
        self.chars.clone().nth(1).unwrap_or('\0')
    }

    fn third(&self) -> char {
        self.chars.clone().nth(2).unwrap_or('\0')
    }

    fn next_char(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    fn skip_at_m(&mut self) {
        while self.first() != 'm' && !self.is_at_end() {
            self.next_char();
        }
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
        let input = read_to_string("./input/day3/1_exemple.txt").unwrap();
        assert_eq!(solver(&input), 161)
    }
}
