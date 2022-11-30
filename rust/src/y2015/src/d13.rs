use std::collections::{HashMap, HashSet};

use aoc::{AocSolution, Solution};
use fancy_regex::Regex;
use itertools::Itertools;
use utils::inputs::Captures;

pub struct Day13;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Person(String);

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Pairing(Person, Person);

fn max_seating_plan_utility(people: &HashSet<Person>, pairings: &HashMap<Pairing, i32>) -> i32 {
    let number_of_people = people.len();
    let seating_plans = people.iter().permutations(number_of_people);

    seating_plans
        .map(|plan| {
            let mut wrapped_plan = plan.into_iter().cloned().collect_vec();
            wrapped_plan.push(wrapped_plan[0].clone());

            wrapped_plan
                .windows(2)
                .map(|window| {
                    let pairing = Pairing(window[0].clone(), window[1].clone());
                    let opposite_pairing = Pairing(window[1].clone(), window[0].clone());
                    pairings[&pairing] + pairings[&opposite_pairing]
                })
                .sum()
        })
        .max()
        .unwrap()
}

impl AocSolution for Day13 {
    fn get_input() -> &'static str {
        include_str!("d13.in")
    }

    type Input = (HashSet<Person>, HashMap<Pairing, i32>);
    fn process_input(input: &str) -> Self::Input {
        let mut pairings: HashMap<Pairing, i32> = HashMap::new();
        let mut people: HashSet<Person> = HashSet::new();

        let line_regex =
            Regex::new(r#"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$"#)
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

            people.insert(person.clone());
            people.insert(adjacent_person.clone());
            pairings.insert(Pairing(person, adjacent_person), points * points_multiplier);
        }

        (people, pairings)
    }

    type Part1Output = i32;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(733);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let (people, pairings) = input;

        max_seating_plan_utility(people, pairings)
    }

    type Part2Output = i32;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Wip;
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let input = input.clone();
        let (mut people, mut pairings) = input;

        let me = Person("me".to_owned());
        let pairings_with_me = people
            .iter()
            .flat_map(|p| {
                [
                    Pairing(me.clone(), p.clone()),
                    Pairing(p.clone(), me.clone()),
                ]
            })
            .collect_vec();

        people.insert(me);
        for p in pairings_with_me {
            pairings.insert(p, 0);
        }

        max_seating_plan_utility(&people, &pairings)
    }
}
