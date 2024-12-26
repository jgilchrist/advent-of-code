use aoc::prelude::*;
use utils::prelude::*;

pub struct Day01;

const DIGITS_REGEX: &str = r"(\d|one|two|three|four|five|six|seven|eight|nine)";

fn to_digit(s: &str) -> u32 {
    if s.len() == 1 {
        let char = s.chars().next().unwrap();
        assert!(char.is_ascii_digit());
        return char.to_digit(10).unwrap();
    }

    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

impl AocSolution for Day01 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.trim().to_string()
    }

    const PART1_SOLUTION: SolutionStatus = solution(55208);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let lines = inputs::lines(input)
            .iter()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let relevant_digits = lines
            .iter()
            .map(|l| (l.first().unwrap(), l.last().unwrap()));

        relevant_digits
            .map(|(d1, d2)| format!("{d1}{d2}"))
            .map(|s| s.parse::<u32>().unwrap())
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(54578);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let forward_captures = inputs::match_per_line(input, DIGITS_REGEX)
            .map(|mut l| l.next_string())
            .map(|s| to_digit(&s))
            .collect::<Vec<_>>();

        let reverse_regex = DIGITS_REGEX
            .chars()
            .rev()
            .collect::<String>()
            .replace('(', "@")
            .replace(')', "(")
            .replace('@', ")")
            .replace("d\\", "\\d");

        let reversed_input = inputs::lines(input)
            .iter()
            .map(|l| l.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        let backward_captures = inputs::match_per_line(&reversed_input, &reverse_regex)
            .map(|mut l| l.next_string().chars().rev().collect::<String>())
            .map(|s| to_digit(&s));

        forward_captures
            .iter()
            .zip(backward_captures)
            .map(|(d1, d2)| format!("{d1}{d2}"))
            .map(|s| s.parse::<u32>().unwrap())
            .sum::<u32>()
    }
}
