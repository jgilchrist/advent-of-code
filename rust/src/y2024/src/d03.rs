use aoc::prelude::*;
use utils::prelude::*;

pub struct Day03;

enum Command {
    Do,
    Dont,
    Mul(u32, u32),
}

impl AocSolution for Day03 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    const PART1_SOLUTION: SolutionStatus = solution(159892596);
    fn part1(input: &Self::Input) -> impl ToSolution {
        inputs::matches_in_str(input, r"mul\((\d+),(\d+)\)")
            .map(|mut c| (c.next_u32(), c.next_u32()))
            .map(|(a, b)| a * b)
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(92626942);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let commands = inputs::matches_in_str(input, r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))")
            .map(|mut c| match c.full() {
                "do()" => Command::Do,
                "don't()" => Command::Dont,
                _ => {
                    // Skip the full match, which is at position 1
                    c.next_string();
                    Command::Mul(c.next_u32(), c.next_u32())
                }
            })
            .collect_vec();

        let mut sum = 0;
        let mut do_multiplies = true;

        for command in commands {
            match command {
                Command::Do => do_multiplies = true,
                Command::Dont => do_multiplies = false,
                Command::Mul(a, b) => {
                    if do_multiplies {
                        sum += a * b;
                    }
                }
            }
        }

        sum
    }
}
