use aoc::prelude::*;
use utils::prelude::*;

pub struct Day11;

type MonkeyId = u8;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn apply(&self, n: u64) -> u64 {
        match self {
            Self::Add(a) => n.checked_add(*a).unwrap(),
            Self::Mul(m) => n.checked_mul(*m).unwrap(),
            Self::Square => n.checked_mul(n).unwrap(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Monkey {
    id: MonkeyId,
    operation: Operation,
    test_divisible_by: u64,
    true_monkey: MonkeyId,
    false_monkey: MonkeyId,
}

impl Monkey {
    fn passes_to(&self, item: u64) -> MonkeyId {
        if item % self.test_divisible_by == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

impl AocSolution for Day11 {
    type Input = Map<MonkeyId, (Monkey, Vec<u64>)>;
    fn process_input(input: &str) -> Self::Input {
        input.split("\n\n").map(parse_monkey).collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(316888);
    fn part1(input: &Self::Input) -> impl ToSolution {
        play_rounds(input, 20, true)
    }

    const PART2_SOLUTION: SolutionStatus = solution(35270398814i64);
    fn part2(input: &Self::Input) -> impl ToSolution {
        play_rounds(input, 10000, false)
    }
}

fn play_rounds(
    input: &Map<MonkeyId, (Monkey, Vec<u64>)>,
    n_rounds: u64,
    divide_by_three: bool,
) -> u64 {
    let monkey_defs: Map<MonkeyId, Monkey> = input
        .iter()
        .map(|(k, (monkey_def, _))| (*k, monkey_def.clone()))
        .collect();

    let mut state: Map<MonkeyId, Vec<u64>> = input
        .iter()
        .map(|(k, (_, held_items))| (*k, held_items.clone()))
        .collect();

    let mut monkey_tests: Map<MonkeyId, u64> = state.keys().map(|&k| (k, 0)).collect();

    for _ in 0..n_rounds {
        state = play_round(&monkey_defs, &state, &mut monkey_tests, divide_by_three);
    }

    let mut test_numbers = monkey_tests.values().copied().collect_vec();
    test_numbers.sort_unstable();
    test_numbers.reverse();

    test_numbers.iter().take(2).product::<u64>()
}

fn play_round(
    monkey_defs: &Map<MonkeyId, Monkey>,
    state: &Map<MonkeyId, Vec<u64>>,
    monkey_tests: &mut Map<MonkeyId, u64>,
    divide_by_three: bool,
) -> Map<MonkeyId, Vec<u64>> {
    let mut state = state.clone();
    let max_monkey_id = *monkey_defs.keys().max().unwrap();
    let divisible_by_all_of = monkey_defs
        .iter()
        .map(|(_, m)| m.test_divisible_by)
        .product::<u64>();

    for monkey_id in 0..=max_monkey_id {
        let monkey = monkey_defs.get(&monkey_id).unwrap();
        while let Some(item) = state.get_mut(&monkey_id).unwrap().pop() {
            let mut item = monkey.operation.apply(item);

            if divide_by_three {
                item /= 3;
            } else {
                item %= divisible_by_all_of;
            }

            let pass_to_monkey = monkey.passes_to(item);
            *monkey_tests.get_mut(&monkey_id).unwrap() += 1;
            state.get_mut(&pass_to_monkey).unwrap().push(item);
        }
    }

    state
}

fn parse_monkey(m: &str) -> (MonkeyId, (Monkey, Vec<u64>)) {
    let lines = m.lines().collect_vec();
    let [monkey_id_line, starting_items_line, operation_line, test_line, true_line, false_line] =
        lines[..]
    else {
        unreachable!()
    };

    let [id] = inputs::n_positive_numbers(monkey_id_line);
    let starting_items: Vec<u64> = inputs::positive_numbers(starting_items_line)
        .iter()
        .map(|&m| m.into())
        .collect();
    let operation = parse_operation(operation_line);
    let [test_divisible_by] = inputs::n_positive_numbers(test_line);
    let [true_monkey] = inputs::n_positive_numbers(true_line);
    let [false_monkey] = inputs::n_positive_numbers(false_line);

    let monkey = Monkey {
        id: id as MonkeyId,
        operation,
        test_divisible_by: test_divisible_by.into(),
        true_monkey: true_monkey as MonkeyId,
        false_monkey: false_monkey as MonkeyId,
    };
    (monkey.id, (monkey, starting_items))
}

fn parse_operation(operation_line: &str) -> Operation {
    if operation_line.contains("* old") {
        return Operation::Square;
    }

    let [n] = inputs::n_positive_numbers(operation_line);

    if operation_line.contains('+') {
        Operation::Add(n.into())
    } else {
        Operation::Mul(n.into())
    }
}
