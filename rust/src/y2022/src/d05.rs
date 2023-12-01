use prelude::*;
use utils::iters::transpose;

pub struct Day05;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug, Clone)]
pub struct Move {
    pub amount: u32,
    pub from_idx: usize,
    pub to_idx: usize,
}

fn top_crates(crates: &[Stack]) -> String {
    crates
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .join("")
}

impl AocSolution for Day05 {
    type Input = (Vec<Stack>, Vec<Move>);
    fn process_input(input: &str) -> Self::Input {
        let (crates_initial_state, moves) = input.split_once("\n\n").unwrap();

        let crates = crates_initial_state
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        // Put the crates in the same stack on the same line of text
        let crates = transpose(&crates);

        let crates = crates
            .iter()
            // Get rid of lines that are just [ or ], or all whitespace
            .filter(|l| {
                !l.iter()
                    .all(|c| c.is_whitespace() || *c == '[' || *c == ']')
            })
            // Make the crate at the bottom of the stack first in the line
            .map(|l| l.iter().rev().collect_vec())
            // Skip the stack number and get rid of any trailing whitespace
            .map(|l| {
                l.into_iter()
                    .skip(1)
                    .filter(|c| !c.is_whitespace())
                    .copied()
                    .collect_vec()
            })
            .collect_vec();

        let moves = moves.lines().map(|l| {
            let [amount, from_number, to_number] = inputs::n_positive_numbers(l);
            Move {
                amount,
                from_idx: (from_number - 1) as usize,
                to_idx: (to_number - 1) as usize,
            }
        });

        (crates, moves.collect())
    }

    const PART1_SOLUTION: Solution = solution("TWSGQHNHL");
    fn part1(input: &Self::Input) -> impl ToSolution {
        let (mut crates, moves) = input.clone();

        for mv in moves {
            for _ in 0..mv.amount {
                let c = crates[mv.from_idx].pop().unwrap();
                crates[mv.to_idx].push(c);
            }
        }

        top_crates(&crates)
    }

    const PART2_SOLUTION: Solution = solution("JNRSCDWPP");
    fn part2(input: &Self::Input) -> impl ToSolution {
        let (mut crates, moves) = input.clone();

        for mv in moves {
            let mut crates_to_move: Vec<Crate> = vec![];
            for _ in 0..mv.amount {
                crates_to_move.push(crates[mv.from_idx].pop().unwrap());
            }
            crates_to_move.reverse();
            crates[mv.to_idx].extend_from_slice(&crates_to_move);
        }

        top_crates(&crates)
    }
}
