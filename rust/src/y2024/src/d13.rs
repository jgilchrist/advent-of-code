use aoc::prelude::*;
use utils::prelude::*;

pub struct Day13;

// Solve a system of two linear equations using Cramer's rule
fn solve(eq1: Equation, eq2: Equation) -> Option<(i64, i64)> {
    let (a1, b1, t1) = eq1;
    let (a2, b2, t2) = eq2;

    // Calculate D:
    let d = a1 * b2 - a2 * b1;

    // Calculate D_a:
    let da = t1 * b2 - t2 * b1;

    // Calculate D_b:
    let db = a1 * t2 - a2 * t1;

    let a = da / d;
    let b = db / d;

    // Check if we actually got a correct solution by feeding the values back
    // into the original equations
    let check = a1 * a + b1 * b == t1 && a2 * a + b2 * b == t2;

    if !check {
        return None;
    }

    Some((a, b))
}

type Equation = (i64, i64, i64);

impl AocSolution for Day13 {
    type Input = Vec<(Equation, Equation)>;
    fn process_input(input: &str) -> Self::Input {
        input
            .split("\n\n")
            .map(|problem| {
                let mut lines = problem.lines();
                let [xa, ya] = inputs::n_numbers(lines.next().unwrap());
                let [xb, yb] = inputs::n_numbers(lines.next().unwrap());
                let [xt, yt] = inputs::n_numbers(lines.next().unwrap());

                (
                    (i64::from(xa), i64::from(xb), i64::from(xt)),
                    (i64::from(ya), i64::from(yb), i64::from(yt)),
                )
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(35729);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .filter_map(|(eq1, eq2)| solve(*eq1, *eq2))
            .map(|(a, b)| a * 3 + b)
            .sum::<i64>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(88584689879723i64);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .map(|(eq1, eq2)| {
                let ((xa, xb, xt), (ya, yb, yt)) = (*eq1, *eq2);
                ((xa, xb, 10000000000000 + xt), (ya, yb, 10000000000000 + yt))
            })
            .filter_map(|(eq1, eq2)| solve(eq1, eq2))
            .map(|(a, b)| a * 3 + b)
            .sum::<i64>()
    }
}
