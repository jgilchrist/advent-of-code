use aoc::prelude::*;
use utils::geometry::d2::grid::Grid;
use utils::prelude::*;

pub struct Day08;

impl AocSolution for Day08 {
    type Input = Grid<Option<char>>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<char>(input).map_cells(|_, c| if *c == '.' { None } else { Some(*c) })
    }

    const PART1_SOLUTION: SolutionStatus = solution(303);
    fn part1(grid: &Self::Input) -> impl ToSolution {
        let mut frequencies = Set::<char>::new();

        // Build up a list of all of the frequencies
        grid.iter_cells().for_each(|(_, c)| {
            if let Some(c) = c {
                frequencies.insert(*c);
            }
        });

        let mut antinode_grid = grid.map_cells(|_, _| false);

        for frequency in frequencies {
            // Get all of the nodes of this frequency on the grid
            let node_positions = grid.cells_matching(|c| *c == Some(frequency));

            // For each pair of nodes...
            for (&node_a, &node_b) in node_positions.iter().tuple_combinations() {
                let dist = node_a - node_b;

                let antinode_1 = node_a + dist;
                let antinode_2 = node_b - dist;

                antinode_grid.set_at_safe(antinode_1, true);
                antinode_grid.set_at_safe(antinode_2, true);
            }
        }

        antinode_grid.cells_matching(|c| *c).len()
    }

    const PART2_SOLUTION: SolutionStatus = solution(1045);
    fn part2(grid: &Self::Input) -> impl ToSolution {
        let mut frequencies = Set::<char>::new();

        // Build up a list of all of the frequencies
        grid.iter_cells().for_each(|(_, c)| {
            if let Some(c) = c {
                frequencies.insert(*c);
            }
        });

        let mut antinode_grid = grid.map_cells(|_, _| false);

        for frequency in frequencies {
            // Get all of the nodes of this frequency on the grid
            let node_positions = grid.cells_matching(|c| *c == Some(frequency));

            // For each pair of nodes...
            for (&node_a, &node_b) in node_positions.iter().tuple_combinations() {
                let dist = node_a - node_b;

                let mut antinode_1 = node_a;

                while antinode_grid.is_valid_coord(&antinode_1) {
                    antinode_1 -= dist;
                    antinode_grid.set_at_safe(antinode_1, true);
                }

                let mut antinode_2 = node_b;

                while antinode_grid.is_valid_coord(&antinode_2) {
                    antinode_2 += dist;
                    antinode_grid.set_at_safe(antinode_2, true);
                }
            }
        }

        antinode_grid.cells_matching(|c| *c).len()
    }
}
