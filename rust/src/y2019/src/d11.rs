use prelude::*;

pub struct Day11;

impl AocSolution for Day11 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.lines().collect()
    }

    const PART1_SOLUTION: Solution = solution(2594);
    const PART1_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part1(_input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }

    const PART2_SOLUTION: Solution = solution(
        r#"
.##..#..#.####.###....##.####.#..#.#..#
#..#.#.#..#....#..#....#.#....#..#.#.#.
#..#.##...###..#..#....#.###..####.##..
####.#.#..#....###.....#.#....#..#.#.#.
#..#.#.#..#....#.#..#..#.#....#..#.#.#.
#..#.#..#.####.#..#..##..#....#..#.#..#"#,
    );
    const PART2_STATUS: SolutionStatus = SolutionStatus::SolvedInPython;
    fn part2(_input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
