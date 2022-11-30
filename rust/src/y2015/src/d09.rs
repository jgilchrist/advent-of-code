use std::collections::HashMap;

use aoc::{AocSolution, Solution};

use itertools::Itertools;
use utils::inputs::regex_lines;

pub struct Day09;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct City(String);

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Route(City, City);

fn calculate_journey_distance(route: &[&City], distances: &HashMap<Route, u32>) -> u32 {
    route
        .windows(2)
        .map(|window| {
            distances
                .get(&Route(window[0].clone(), window[1].clone()))
                .unwrap()
        })
        .sum()
}

fn calculate_all_journey_distances(distances: &HashMap<Route, u32>) -> Vec<u32> {
    let all_cities = distances
        .keys()
        .map(|k| k.0.clone())
        .unique_by(|k| k.0.clone())
        .collect_vec();

    let all_routes = all_cities.iter().permutations(all_cities.len());

    all_routes
        .map(|route| calculate_journey_distance(&route, distances))
        .collect()
}

impl AocSolution for Day09 {
    fn get_input() -> &'static str {
        include_str!("d09.in")
    }

    type Input = HashMap<Route, u32>;
    fn process_input(input: &str) -> Self::Input {
        HashMap::from_iter(
            regex_lines(input, r#"(\w+) to (\w+) = (\d+)"#).flat_map(|line| {
                let from_city = City(line.get_string(1));
                let to_city = City(line.get_string(2));
                let distance = line.get_u32(3);

                [
                    (Route(from_city.clone(), to_city.clone()), distance),
                    (Route(to_city.clone(), from_city.clone()), distance),
                ]
            }),
        )
    }

    type Part1Output = u32;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(207);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        *calculate_all_journey_distances(input).iter().min().unwrap()
    }

    type Part2Output = u32;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(804);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        *calculate_all_journey_distances(input).iter().max().unwrap()
    }
}
