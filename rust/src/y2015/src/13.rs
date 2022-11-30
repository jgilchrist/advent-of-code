use std::collections::HashMap;

use aoc::{AocSolution, Solution};
use fancy_regex::Regex;
use utils::inputs::Captures;

pub struct Day13;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Person(String);

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Pairing(Person, Person);

fn calculate_all_seating_plan_utilities(pairings: HashMap<Pairing, i32>) -> i32 {
    // let seating_plans =
}

impl AocSolution for Day13 {
    fn get_input() -> &'static str {
        include_str!("d13.in")
    }

    type Input = HashMap<Pairing, i32>;
    fn process_input(input: &str) -> Self::Input {
        let mut pairings: HashMap<Pairing, i32> = HashMap::new();

        let line_regex =
            Regex::new(r#"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)$"#)
                .unwrap();

        for line in input.lines() {
            let captures = Captures(line_regex.captures(line).unwrap().unwrap());

            let person = Person(captures.get_string(1));
            let action = captures.get_string(2);
            let points = captures.get_i32(3);
            let adjacent_person = Person(captures.get_string(4));

            let points_multiplier: i32 = match action.as_str() {
                "gain" => 1,
                "lose" => -1,
                _ => unreachable!(),
            };

            pairings.insert(Pairing(person, adjacent_person), points * points_multiplier);
        }

        pairings
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
