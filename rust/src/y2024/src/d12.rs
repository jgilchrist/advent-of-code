use aoc::prelude::*;
use utils::geometry::d2::coordinates::PrincipalWinds;
use utils::geometry::d2::grid::Grid;
use utils::geometry::d2::vecs::Vec2;
use utils::prelude::*;

pub struct Day12;

impl AocSolution for Day12 {
    type Input = Grid<Option<char>>;
    fn process_input(input: &str) -> Self::Input {
        inputs::grid_of::<char>(input)
            .map_cells(|_, cell| Some(*cell))
            .with_border(None)
    }

    const PART1_SOLUTION: SolutionStatus = solution(1465112);
    fn part1(grid: &Self::Input) -> impl ToSolution {
        let regions = calculate_regions(grid);

        regions
            .iter()
            .map(|(_, vs)| perimeter(grid, vs) * vs.len())
            .sum::<usize>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(893790);
    fn part2(grid: &Self::Input) -> impl ToSolution {
        let regions = calculate_regions(grid);

        regions
            .iter()
            .map(|(_, vs)| sides(grid, vs) * vs.len())
            .sum::<usize>()
    }
}

fn calculate_regions(grid: &Grid<Option<char>>) -> Vec<(char, Set<Vec2>)> {
    let mut seen_squares: Set<Vec2> = Set::new();
    let mut regions: Vec<(char, Set<Vec2>)> = Vec::new();

    for (pos, char) in grid.iter_cells() {
        // Skip any squares we've already made part of a region
        if seen_squares.contains(&pos) {
            continue;
        }

        let Some(char) = char else {
            continue;
        };

        let region = fill_region(grid, pos);

        regions.push((*char, region.clone()));

        seen_squares = seen_squares.union(&region).copied().collect();
    }

    regions
}

fn fill_region(grid: &Grid<Option<char>>, start: Vec2) -> Set<Vec2> {
    let region_char = grid.at(start).unwrap();

    let mut region_squares = Set::new();
    region_squares.insert(start);

    // Continually loop, expanding along the boundary of our current region until we can't go any further
    loop {
        let boundary = region_squares
            .iter()
            .flat_map(|c| grid.neighbors4(*c))
            .unique()
            .filter(|p| grid.at(*p).unwrap() == region_char)
            .collect::<Set<_>>();

        let new_region_squares = region_squares.union(&boundary).copied().collect::<Set<_>>();

        if new_region_squares == region_squares {
            return region_squares;
        }

        region_squares = new_region_squares;
    }
}

fn perimeter(grid: &Grid<Option<char>>, vs: &Set<Vec2>) -> usize {
    vs.iter()
        .map(|pos|
             // For each position in the region, look at each neighbors and pick
             // ones that are not in the grid, or not in the region
            grid.neighbors4(*pos)
                .filter(|newpos| !vs.contains(newpos))
                .count())
        .sum()
}

fn is_corner_for_direction(
    grid: &Grid<Option<char>>,
    points: &Set<Vec2>,
    pos: Vec2,
    corner_dir: PrincipalWinds,
) -> bool {
    // Determine the neighbour squares we're looking at.
    let [dir1, dir2] = corner_dir.components().unwrap();
    let neighbor1 = grid.move_pos_in_direction(pos, dir1);
    let neighbor2 = grid.move_pos_in_direction(pos, dir2);
    let corner = grid.move_pos_in_direction(pos, corner_dir);

    let neighbor1_is_in_region = neighbor1.is_some() && points.contains(&neighbor1.unwrap());
    let neighbor2_is_in_region = neighbor2.is_some() && points.contains(&neighbor2.unwrap());
    let corner_is_in_region = corner.is_some() && points.contains(&corner.unwrap());

    // An outside corner:
    //     *
    //    *+.    + is an outside corner when checking north west since both north & west squares
    //     .     are in the region
    let is_outside_corner = !neighbor1_is_in_region && !neighbor2_is_in_region;

    // An inside corner:
    //
    //     +.    + is an inside corner when checking south east since south & east squares are
    //     .*    are in the region, but the square between them diagonally is not
    let is_inside_corner = neighbor1_is_in_region && neighbor2_is_in_region && !corner_is_in_region;

    is_outside_corner || is_inside_corner
}

fn sides(grid: &Grid<Option<char>>, vs: &Set<Vec2>) -> usize {
    vs.iter()
        .map(|pos| {
            PrincipalWinds::DIAGONALS
                .into_iter()
                .filter(|dir| is_corner_for_direction(grid, vs, *pos, *dir))
                .count()
        })
        .sum()
}
