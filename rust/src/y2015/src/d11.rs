use prelude::*;

pub struct Day11;

#[allow(clippy::as_conversions)]
fn increment_letter(c: char) -> (char, bool) {
    let index_into_alphabet = c as u8 - 97;
    let incremented_and_wrapped = (index_into_alphabet + 1) % 26;
    let new_letter = (incremented_and_wrapped + 97) as char;
    let wrapped = incremented_and_wrapped == 0;

    (new_letter, wrapped)
}

fn increment_password(password: &str) -> String {
    let mut password_chars = password.chars().collect_vec();
    let mut carry = true;
    let mut idx = password.len() - 1;

    while carry {
        let current_letter = password_chars[idx];
        let (new_letter, new_carry) = increment_letter(current_letter);
        carry = new_carry;
        password_chars[idx] = new_letter;
        idx -= 1;
    }

    password_chars.iter().map(|c| c.to_string()).join("")
}

fn has_pairs(password: &str) -> bool {
    let mut chars = password.chars().collect_vec();
    chars.push(' ');

    let mut result = String::new();
    let mut count = 1;

    for window in chars.windows(2) {
        let &[last_char, current_char] = window else { unreachable!() };

        if current_char == last_char {
            count += 1;
        } else {
            if count > 1 {
                result.push_str(&last_char.to_string());
            }

            count = 1;
        }
    }

    result.len() >= 2
}

#[allow(clippy::as_conversions)]
fn has_incrementing_character_run(password: &str) -> bool {
    let chars = password.chars().collect_vec();

    for window in chars.windows(3) {
        let &[a, b, c] = window else { unreachable!() };
        let a = a as u8;
        let b = b as u8;
        let c = c as u8;

        if a == b - 1 && a == c - 2 {
            return true;
        }
    }

    false
}

fn contains_invalid_chars(password: &str) -> bool {
    password.contains('i') || password.contains('o') || password.contains('l')
}

fn is_valid_password(password: &str) -> bool {
    has_incrementing_character_run(password)
        && !contains_invalid_chars(password)
        && has_pairs(password)
}

fn next_valid_password(password: &str) -> String {
    let mut password = password.to_owned();

    while !is_valid_password(&password) {
        password = increment_password(&password);
    }

    password
}

impl AocSolution for Day11 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.trim().to_owned()
    }

    const PART1_SOLUTION: Solution = solution("cqjxxyzz");
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        next_valid_password(input)
    }

    const PART2_SOLUTION: Solution = solution("cqkaabcc");
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let new_password = next_valid_password(input);
        next_valid_password(&increment_password(&new_password))
    }
}
