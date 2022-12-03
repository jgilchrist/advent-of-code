use prelude::*;

pub struct Day10;

fn compress_number(number: &str) -> String {
    let mut chars = number.chars().collect_vec();
    chars.push(' ');

    let mut result = String::new();
    let mut count = 1;

    for &[last_char, current_char] in chars.array_windows() {
        if current_char == last_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push_str(&last_char.to_string());
            count = 1;
        }
    }

    result
}

impl AocSolution for Day10 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.trim().to_owned()
    }

    const PART1_SOLUTION: Solution = solution(252594);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        let mut s = input.clone();

        for _ in 0..40 {
            s = compress_number(&s);
        }

        s.len()
    }

    const PART2_SOLUTION: Solution = solution(3579328);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let mut s = input.clone();

        for _ in 0..50 {
            s = compress_number(&s);
        }

        s.len()
    }
}
