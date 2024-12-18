use aoc::prelude::*;

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
    type Input = serde_json::Value;
    fn process_input(input: &str) -> Self::Input {
        serde_json::from_str(input).unwrap()
    }

    const PART1_SOLUTION: SolutionStatus = solution(119433);
    fn part1(input: &Self::Input) -> impl ToSolution {
        count_numbers(input)
    }

    const PART2_SOLUTION: SolutionStatus = solution(68466);
    fn part2(input: &Self::Input) -> impl ToSolution {
        count_numbers_ignoring_red(input)
    }
}
