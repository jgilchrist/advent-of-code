use prelude::*;
use utils::geometry::d2::{coordinates::CardinalDirection, map::Map};
use utils::iters::TakeUntilExt;

pub struct Day08;

impl AocSolution for Day08 {
    type Input = Map<u32>;
    fn process_input(input: &str) -> Self::Input {
        Map::new(
            input
                .lines()
                .join("")
                .chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect(),
        )
    }

    const PART1_SOLUTION: Solution = solution(1717);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        input
            .iter_cells()
            .filter(|(tree_coord, tree_height)| {
                CardinalDirection::all()
                    .map(|direction| input.raycast_cells(*tree_coord, direction).collect_vec())
                    .any(|tree_heights| {
                        tree_heights.is_empty()
                            || tree_heights
                                .iter()
                                .all(|other_tree_height| other_tree_height < tree_height)
                    })
            })
            .count()
    }

    const PART2_SOLUTION: Solution = solution(321975);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        input
            .iter_cells()
            .map(|(tree_coord, tree_height)| {
                CardinalDirection::all()
                    .map(|direction| input.raycast_cells(tree_coord, direction))
                    .map(|heights| heights.take_until(|h| *h >= tree_height).count())
                    .product::<usize>()
            })
            .max()
            .unwrap()
    }
}
