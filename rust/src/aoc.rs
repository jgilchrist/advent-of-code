pub type TestDefinition<Output> = (&'static str, Option<Output>, Option<Output>);

pub enum Solution<T> {
    Solved(T),
    Wip,
    WipWithKnownAnswerFromPython(T),
    Unsolved,
    UnsolvedWithKnownAnswerFromPython(T),
    MerryChristmas,
}

impl<T> Solution<T> {
    pub fn has_solution(&self) -> bool {
        use Solution::*;
        match *self {
            Solved(_)
            | Wip
            | WipWithKnownAnswerFromPython(_)
            | MerryChristmas => true,
            Unsolved | UnsolvedWithKnownAnswerFromPython(_) => false,
        }
    }
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
        unimplemented!()
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Unsolved;
    fn part2(_: &Self::Input) -> Self::Part2Output {
        unimplemented!()
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
