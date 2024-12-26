use aoc::prelude::*;
use utils::geometry::d2::{coordinates::CardinalDirection, grid::Grid};
use utils::prelude::*;

pub struct Day08;

impl AocSolution for Day08 {
    type Input = Grid<u32>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<u32>(input)
    }

    const PART1_SOLUTION: SolutionStatus = solution(1717);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter_cells()
            .filter(|(tree_coord, tree_height)| {
                CardinalDirection::ALL
                    .map(|direction| {
                        input
                            .raycast_cells(*tree_coord, direction.into())
                            .collect::<Vec<_>>()
                    })
                    .iter()
                    .any(|tree_heights| {
                        tree_heights.is_empty()
                            || tree_heights
                                .iter()
                                .all(|other_tree_height| other_tree_height < tree_height)
                    })
            })
            .count()
    }

    const PART2_SOLUTION: SolutionStatus = solution(321975);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter_cells()
            .map(|(tree_coord, tree_height)| {
                CardinalDirection::ALL
                    .map(|direction| input.raycast_cells(tree_coord, direction.into()))
                    .map(|heights| heights.take_while_inclusive(|h| *h < tree_height).count())
                    .iter()
                    .product::<usize>()
            })
            .max()
            .unwrap()
    }
}
