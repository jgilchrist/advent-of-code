use aoc::prelude::*;
use utils::prelude::*;

pub struct Day09;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct City(String);

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Route(City, City);

fn calculate_journey_distance(route: &[&City], distances: &Map<Route, u32>) -> u32 {
    route
        .array_windows()
        .map(|&[a, b]| distances.get(&Route(a.clone(), b.clone())).unwrap())
        .sum()
}

fn calculate_all_journey_distances(distances: &Map<Route, u32>) -> Vec<u32> {
    let all_cities = distances
        .keys()
        .map(|k| k.0.clone())
        .unique_by(|k| k.0.clone())
        .collect::<Vec<_>>();

    let all_routes = all_cities.iter().permutations(all_cities.len());

    all_routes
        .map(|route| calculate_journey_distance(&route, distances))
        .collect()
}

impl AocSolution for Day09 {
    type Input = Map<Route, u32>;
    fn process_input(input: &str) -> Self::Input {
        inputs::match_per_line(input, r"(\w+) to (\w+) = (\d+)")
            .flat_map(|mut line| {
                let from_city = City(line.next_string());
                let to_city = City(line.next_string());
                let distance = line.next_u32();

                [
                    (Route(from_city.clone(), to_city.clone()), distance),
                    (Route(to_city, from_city), distance),
                ]
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(207);
    fn part1(input: &Self::Input) -> impl ToSolution {
        *calculate_all_journey_distances(input).iter().min().unwrap()
    }

    const PART2_SOLUTION: SolutionStatus = solution(804);
    fn part2(input: &Self::Input) -> impl ToSolution {
        *calculate_all_journey_distances(input).iter().max().unwrap()
    }
}
