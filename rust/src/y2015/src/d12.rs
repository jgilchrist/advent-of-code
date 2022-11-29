use aoc::{AocSolution, Solution};

pub struct Day12;

fn count_numbers(json: &serde_json::Value) -> i32 {
    match json {
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => 0,
        serde_json::Value::Number(n) => n.as_i64().unwrap().try_into().unwrap(),
        serde_json::Value::Array(elems) => elems.iter().map(count_numbers).sum(),
        serde_json::Value::Object(obj) => obj
            .iter()
            .map(|(_, prop_value)| count_numbers(prop_value))
            .sum(),
    }
}

fn count_numbers_ignoring_red(json: &serde_json::Value) -> i32 {
    match json {
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => 0,
        serde_json::Value::Number(n) => n.as_i64().unwrap().try_into().unwrap(),
        serde_json::Value::Array(elems) => elems.iter().map(count_numbers_ignoring_red).sum(),
        serde_json::Value::Object(obj) => {
            let contains_red = obj
                .iter()
                .any(|(_, value)| value.is_string() && value.as_str().unwrap() == "red");

            if contains_red {
                0
            } else {
                obj.into_iter()
                    .map(|(_, prop_value)| count_numbers_ignoring_red(prop_value))
                    .sum()
            }
        }
    }
}

impl AocSolution for Day12 {
    fn get_input() -> &'static str {
        include_str!("d12.in")
    }

    type Input = serde_json::Value;
    fn process_input(input: &str) -> Self::Input {
        serde_json::from_str(input).unwrap()
    }

    type Part1Output = i32;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(119433);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        count_numbers(input)
    }

    type Part2Output = i32;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(68466);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        count_numbers_ignoring_red(input)
    }
}
