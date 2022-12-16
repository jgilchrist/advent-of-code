use prelude::*;
use utils::geometry::d2::{vecs::Vec2, coordinates::CardinalDirection};

pub struct Day15;

struct Diamond {
    center: Vec2,
    height: u32,
}

impl Diamond {
    fn coords(&self) -> HashSet<Vec2> {
        let mut slice_center = self.center.move_in_direction_by(CardinalDirection::North, self.height);
        let mut width = 0;
        let mut points: HashSet<Vec2> = HashSet::new();

        while slice_center.y != self.center.y {
            let new_points = (slice_center.x - width ..= slice_center.x + width).map(|x| Vec2::new(x, slice_center.y)).collect_vec();
            points.extend(new_points.into_iter());
            width += 1;
            slice_center = slice_center.move_in_direction(CardinalDirection::South);
        }

        let new_points = (slice_center.x - width ..= slice_center.x + width).map(|x| Vec2::new(x, slice_center.y)).collect_vec();
        points.extend(new_points.into_iter());
        width -= 1;
        slice_center = slice_center.move_in_direction(CardinalDirection::South);

        while width >= 0 {
            let new_points = (slice_center.x - width ..= slice_center.x + width).map(|x| Vec2::new(x, slice_center.y)).collect_vec();
            points.extend(new_points.into_iter());
            width -= 1;
            slice_center = slice_center.move_in_direction(CardinalDirection::South);
        }

        points
    }

    fn from_center_and_point_on_boundary(center: Vec2, point_on_boundary: Vec2) -> Self {
    }
}

impl AocSolution for Day15 {
    type Input = String;
    fn process_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    const PART1_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART1_SOLUTION: Solution = Solution::Unsolved;
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        dbg!(Diamond {
            center: Vec2::new(0, 0),
            height: 2,
        }.coords());

        todo!();
        0
    }

    const PART2_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART2_SOLUTION: Solution = Solution::Unsolved;
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
