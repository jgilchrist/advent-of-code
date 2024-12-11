use aoc::prelude::*;
use utils::geometry::d2::coordinates::PrincipalWinds;
use utils::geometry::d2::grid::Grid;
use utils::prelude::*;

pub struct Day04;

impl AocSolution for Day04 {
    type Input = Grid<char>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<char>(input)
    }

    const PART1_SOLUTION: SolutionStatus = solution(2569);
    fn part1(grid: &Self::Input) -> impl ToSolution {
        grid.iter_coords()
            .map(|c| {
                PrincipalWinds::ALL
                    .iter()
                    .filter(|&&dir| {
                        grid.raycast_cells_inclusive(c, dir)
                            .take(4)
                            .collect::<String>()
                            == "XMAS"
                    })
                    .count()
            })
            .sum::<usize>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(1998);
    fn part2(grid: &Self::Input) -> impl ToSolution {
        grid.iter_coords()
            .filter(|c| {
                let diagonal1 = grid
                    .raycast_cells_inclusive(
                        c.move_in_direction(PrincipalWinds::NorthWest),
                        PrincipalWinds::SouthEast,
                    )
                    .take(3)
                    .collect::<String>();

                let diagonal2 = grid
                    .raycast_cells_inclusive(
                        c.move_in_direction(PrincipalWinds::SouthWest),
                        PrincipalWinds::NorthEast,
                    )
                    .take(3)
                    .collect::<String>();

                (diagonal1 == "MAS" || diagonal1 == "SAM")
                    && (diagonal2 == "MAS" || diagonal2 == "SAM")
            })
            .count()
    }
}
