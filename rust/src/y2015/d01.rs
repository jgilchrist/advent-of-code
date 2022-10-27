use crate::{AocSolution, aoc::Solution};

pub struct Day01;

impl AocSolution for Day01 {
    type Input = String;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d01.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.to_string()
    }

    const PART1_SOLUTION: Solution<Self::Output> = Solution::Solution(74);
    fn part1(input: &Self::Input) -> Self::Output {
        input.matches('(').count() - input.matches(')').count()
    }

    const PART2_SOLUTION: Solution<Self::Output> = Solution::Solution(1795);
    fn part2(input: &Self::Input) -> Self::Output {
        let mut floor = 0;

        for (i, c) in input.chars().enumerate() {
            if c == '(' {
                floor += 1
            } else {
                floor -= 1
            }

            if floor == -1 {
                return i + 1;
            }
        }

        unreachable!()
    }
}
