#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::approx_constant)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::too_many_lines)]

pub type TestDefinition<Output> = (&'static str, Option<Output>, Option<Output>);

#[allow(unused)]
pub enum Solution<T> {
    Solved(T),
    Wip,
    WipWithKnownAnswerFromPython(T),
    Unsolved,
    UnsolvedWithKnownAnswerFromPython(T),
    MerryChristmas,
}

pub trait AocSolution {
    fn tests() -> Vec<TestDefinition<Self::Part1Output>> {
        vec![]
    }

    fn get_input() -> &'static str;

    type Input;
    fn process_input(input: &str) -> Self::Input;

    type Part1Output: std::fmt::Display + Eq;
    const PART1_SOLUTION: Solution<Self::Part1Output>;
    fn part1(i: &Self::Input) -> Self::Part1Output;

    type Part2Output: std::fmt::Display + Eq;
    const PART2_SOLUTION: Solution<Self::Part2Output>;
    fn part2(i: &Self::Input) -> Self::Part2Output;
}

pub struct Unsolved;
impl AocSolution for Unsolved {
    type Input = ();

    fn get_input() -> &'static str {
        ""
    }

    fn process_input(_: &str) -> Self::Input {}

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Unsolved;
    fn part1(_: &Self::Input) -> Self::Part1Output {
        0
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Unsolved;
    fn part2(_: &Self::Input) -> Self::Part2Output {
        0
    }
}

pub trait AocYear {
    type D01: AocSolution;
    type D02: AocSolution;
    type D03: AocSolution;
    type D04: AocSolution;
    type D05: AocSolution;
    type D06: AocSolution;
    type D07: AocSolution;
    type D08: AocSolution;
    type D09: AocSolution;
    type D10: AocSolution;
    type D11: AocSolution;
    type D12: AocSolution;
    type D13: AocSolution;
    type D14: AocSolution;
    type D15: AocSolution;
    type D16: AocSolution;
    type D17: AocSolution;
    type D18: AocSolution;
    type D19: AocSolution;
    type D20: AocSolution;
    type D21: AocSolution;
    type D22: AocSolution;
    type D23: AocSolution;
    type D24: AocSolution;
    type D25: AocSolution;
}
