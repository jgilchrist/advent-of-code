use prelude::*;
use utils::iters::transpose;

pub struct Day03;

type Triangle = [u32; 3];

fn is_valid_triangle(t: Triangle) -> bool {
    let mut t = t;
    t.sort_unstable();
    let [smallest, middle, largest] = t;
    smallest + middle > largest
}

fn count_valid_triangles(ts: &[Triangle]) -> usize {
    ts.iter().filter(|t| is_valid_triangle(**t)).count()
}

impl AocSolution for Day03 {
    fn get_input() -> &'static str {
        include_str!("d03.in")
    }

    type Input = Vec<[u32; 3]>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let numbers = inputs::positive_numbers(l);
                let [x1, x2, x3] = numbers[..] else {
                    unreachable!()
                };

                [x1, x2, x3]
            })
            .collect_vec()
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(1050);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        count_valid_triangles(input)
    }

    type Part2Output = usize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(1921);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let slices = input
            .iter()
            .map(|&l| l.into_iter().collect_vec())
            .collect_vec();

        let new_triangles = transpose(&slices)
            .concat()
            .chunks(3)
            .map(|c| {
                let [x1, x2, x3] = c[..] else {
                    unreachable!()
                };

                [x1, x2, x3]
            })
            .collect_vec();

        count_valid_triangles(&new_triangles)
    }
}
