use aoc::prelude::*;
use utils::geometry::d2::{
    coordinates::{CardinalDirection, PrincipalWinds},
    shapes::Line,
    vecs::Vec2,
};
use utils::prelude::*;

pub struct Day14;

const SAND_SPAWN_POS: Vec2 = Vec2::new(500, 0);

impl AocSolution for Day14 {
    type Input = (HashSet<Vec2>, i32);
    fn process_input(input: &str) -> Self::Input {
        let occupied_cells: HashSet<Vec2> = input.lines().flat_map(get_points_in_line).collect();
        let floor_position = occupied_cells.iter().map(|v| v.y).max().unwrap() + 2;

        (occupied_cells, floor_position)
    }

    const PART1_SOLUTION: SolutionStatus = solution(715);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let (mut occupied_cells, floor_position) = input.clone();
        simulate_to_end(&mut occupied_cells, floor_position, false)
    }

    const PART2_SOLUTION: SolutionStatus = solution(25248);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let (mut occupied_cells, floor_position) = input.clone();
        simulate_to_end(&mut occupied_cells, floor_position, true)
    }
}

fn simulate_to_end(
    occupied_cells: &mut HashSet<Vec2>,
    floor_position: i32,
    floor_collision: bool,
) -> u32 {
    let mut units_placed = 0;

    while let Some(pos) = simulate_sand(occupied_cells, floor_position, floor_collision) {
        occupied_cells.insert(pos);
        units_placed += 1;

        if pos == SAND_SPAWN_POS {
            break;
        }
    }

    units_placed
}

fn simulate_sand(
    occupied_cells: &HashSet<Vec2>,
    floor_position: i32,
    floor_collision: bool,
) -> Option<Vec2> {
    let mut sand_position = SAND_SPAWN_POS;

    let can_move_to_cell = |cell: Vec2| {
        if floor_collision && cell.y == floor_position {
            return false;
        }

        !occupied_cells.contains(&cell)
    };

    loop {
        if !floor_collision && sand_position.y == floor_position {
            return None;
        }

        let straight_down = sand_position.move_in_direction(CardinalDirection::South);
        if can_move_to_cell(straight_down) {
            sand_position = straight_down;
            continue;
        }

        let down_and_left = sand_position.move_in_direction(PrincipalWinds::SouthWest);
        if can_move_to_cell(down_and_left) {
            sand_position = down_and_left;
            continue;
        }

        let down_and_right = sand_position.move_in_direction(PrincipalWinds::SouthEast);
        if can_move_to_cell(down_and_right) {
            sand_position = down_and_right;
            continue;
        }

        return Some(sand_position);
    }
}

fn get_points_in_line(s: &str) -> HashSet<Vec2> {
    s.split(" -> ")
        .map(|vertex| {
            let [x, y] = inputs::n_positive_numbers(vertex);
            Vec2::new(x as i32, y as i32)
        })
        .collect_vec()
        .array_windows()
        .flat_map(|[v1, v2]| Line::new(*v1, *v2).points())
        .collect()
}
