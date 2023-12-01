use prelude::*;

pub struct Day20;

impl AocSolution for Day20 {
    type Input = u32;
    fn process_input(input: &str) -> Self::Input {
        input.trim().parse::<u32>().unwrap()
    }

    // Originally attempted to solve with modulos, but it was slower than just
    // allocating a huge array.
    const PART1_SOLUTION: Solution = solution(665280);
    #[allow(clippy::large_stack_arrays, clippy::large_stack_frames)]
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut ns = [0; 1_000_000];

        for i in 1..ns.len() {
            let mut n = i;

            while n < ns.len() {
                ns[n] += 10 * i as u32;
                n += i;
            }
        }

        ns.iter()
            .enumerate()
            .find(|(_, n)| **n >= *input)
            .map(|(i, _)| i)
            .unwrap()
    }

    const PART2_SOLUTION: Solution = solution(705600);
    #[allow(clippy::large_stack_arrays, clippy::large_stack_frames)]
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut ns = [0; 1_000_000];

        for i in 1..ns.len() {
            let mut n = i;
            let mut times_incremented = 0;

            while n < ns.len() && times_incremented < 50 {
                ns[n] += 11 * i as u32;
                n += i;
                times_incremented += 1;
            }
        }

        ns.iter()
            .enumerate()
            .find(|(_, n)| **n >= *input)
            .map(|(i, _)| i)
            .unwrap()
    }
}
