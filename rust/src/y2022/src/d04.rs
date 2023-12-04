use aoc::prelude::*;
use utils::prelude::*;

pub struct Day04;

pub struct Range(u32, u32);

impl Range {
    fn contains_range(&self, other: &Self) -> bool {
        let Self(s1, e1) = self;
        let Self(s2, e2) = other;
        s1 <= s2 && e1 >= e2
    }

    fn contains(&self, n: u32) -> bool {
        let &Self(start, end) = self;
        n >= start && n <= end
    }

    fn overlaps(&self, other: &Self) -> bool {
        let &Self(start, end) = other;
        self.contains(start) || self.contains(end)
    }
}

impl AocSolution for Day04 {
    type Input = Vec<(Range, Range)>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let [s1, e1, s2, e2] = inputs::n_positive_numbers(l);
                (Range(s1, e1), Range(s2, e2))
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(602);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .filter(|(r1, r2)| r1.contains_range(r2) || r2.contains_range(r1))
            .count()
    }

    const PART2_SOLUTION: SolutionStatus = solution(891);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .filter(|(r1, r2)| r1.overlaps(r2) || r2.overlaps(r1))
            .count()
    }
}
