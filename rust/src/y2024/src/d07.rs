use aoc::prelude::*;
use utils::prelude::*;

pub struct Day07;

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

fn can_be_true(def: &(u64, Vec<u32>), allowed_operators: &[Operator]) -> bool {
    let (result, components) = def;

    let (initial, rest) = components.split_at(1);
    let initial = initial.first().unwrap();

    // One operator for each 'gap' between components
    let number_of_operators = rest.len();

    // Make a list of all of the possible permutations of operators that fit in the 'gaps' between our components
    // repeat_n(...).multi_cartesian_product() is the equivalent of .permutations_with_replacement()
    let all_possible_operators = repeat_n(allowed_operators, number_of_operators)
        .multi_cartesian_product()
        .collect_vec();

    for operators in all_possible_operators {
        let mut this_result: u64 = u64::from(*initial);

        for (operator, operand) in operators.iter().zip(rest) {
            match operator {
                Operator::Plus => this_result += u64::from(*operand),
                Operator::Multiply => this_result *= u64::from(*operand),
                Operator::Concatenate => {
                    let this_result_as_str = this_result.to_string();
                    let operand_as_str = operand.to_string();

                    let concatenated_str = this_result_as_str + &operand_as_str;

                    this_result = concatenated_str.parse().unwrap();
                }
            }
        }

        // Only one set of operators needs to work
        if this_result == *result {
            return true;
        }
    }

    false
}

impl AocSolution for Day07 {
    type Input = Vec<(u64, Vec<u32>)>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (result, components) = l.split_once(": ").unwrap();

                let result = result.parse::<u64>().unwrap();
                let components = inputs::positive_numbers(components);
                (result, components)
            })
            .collect_vec()
    }

    const PART1_SOLUTION: SolutionStatus = solution(1430271835320i64);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .filter(|i| can_be_true(i, &[Operator::Plus, Operator::Multiply]))
            .map(|i| i.0)
            .sum::<u64>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(456565678667482i64);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .filter(|i| {
                can_be_true(
                    i,
                    &[Operator::Plus, Operator::Multiply, Operator::Concatenate],
                )
            })
            .map(|i| i.0)
            .sum::<u64>()
    }
}
