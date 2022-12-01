use prelude::*;

pub struct Day01;

impl AocSolution for Day01 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    const PART1_SOLUTION: Solution = solution(74);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        input.matches('(').count() - input.matches(')').count()
    }

    const PART2_SOLUTION: Solution = solution(1795);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let mut floor = 0;

        for (i, c) in input.chars().enumerate() {
            if c == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }

            if floor == -1 {
                return i + 1;
            }
        }

        unreachable!()
    }
}
