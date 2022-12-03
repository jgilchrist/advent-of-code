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
    type Input = Vec<[u32; 3]>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| inputs::n_positive_numbers::<3>(l))
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(1050);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        count_valid_triangles(input)
    }

    const PART2_SOLUTION: Solution = solution(1921);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let slices = input.iter().map(|&l| l.into_iter().collect()).collect_vec();

        let new_triangles: Vec<Triangle> = transpose(&slices)
            .concat()
            .array_chunks()
            .map(|&[x1, x2, x3]| [x1, x2, x3])
            .collect();

        count_valid_triangles(&new_triangles)
    }
}
