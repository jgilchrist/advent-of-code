use aoc::prelude::*;
use utils::geometry::d2::grid::Grid;
use utils::geometry::d2::vecs::Vec2;
use utils::prelude::*;

pub struct Day10;

fn number_of_neighbors_in_trail(grid: &Grid<u32>, s: Vec2, num: u32) -> usize {
    if num == 9 {
        return 1;
    }

    let number_to_find = num + 1;

    grid.neighbor_cells4(s)
        .filter(|(_, n)| **n == number_to_find)
        .map(|(pos, _)| number_of_neighbors_in_trail(grid, pos, number_to_find))
        .sum()
}

fn number_of_trails_from_s(grid: &Grid<u32>, s: Vec2) -> usize {
    number_of_neighbors_in_trail(grid, s, 0)
}

fn all_neighbors(grid: &Grid<u32>, ps: &[Vec2]) -> Vec<Vec2> {
    ps.iter()
        .flat_map(|c| grid.neighbors4(*c))
        .unique()
        .collect_vec()
}

impl AocSolution for Day10 {
    type Input = Grid<u32>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<char>(input).map_cells(|_, c| c.to_digit(10).unwrap())
    }

    const PART1_SOLUTION: SolutionStatus = solution(624);
    fn part1(grid: &Self::Input) -> impl ToSolution {
        grid.cells_matching(|c| *c == 0)
            .iter()
            .map(|start| {
                let mut current_cells = vec![*start];

                // For each number we want to find, look for all neighbors of our current cells that
                // match that number
                for i in 1..=9 {
                    current_cells = all_neighbors(grid, &current_cells)
                        .into_iter()
                        .filter(|p| *grid.at(*p).unwrap() == i)
                        .collect_vec();
                }

                current_cells.len()
            })
            .sum::<usize>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(1483);
    fn part2(grid: &Self::Input) -> impl ToSolution {
        grid.cells_matching(|c| *c == 0)
            .iter()
            .map(|s| number_of_trails_from_s(grid, *s))
            .sum::<usize>()
    }
}
