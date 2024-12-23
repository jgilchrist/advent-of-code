use aoc::prelude::*;

pub struct Day21;

impl AocSolution for Day21 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution_from_python(1679);
    fn part1(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus =
        solution_from_python("lmxt,rggkbpj,mxf,gpxmf,nmtzlj,dlkxsxg,fvqg,dxzq");
    fn part2(_input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
