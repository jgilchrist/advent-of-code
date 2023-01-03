use prelude::*;
use utils::geometry::d2::{vecs::Vec2, shapes::Square};

pub struct Day15;

pub struct Diamond {
    center: Vec2,
    radius: u32,
}

type Sensor = Vec2;
type Beacon = Vec2;

impl Diamond {
    fn coords(&self) -> impl Iterator<Item = Vec2> + '_ {
        let top_left = self.center - Vec2::new(self.radius as i32 - 1, self.radius as i32 - 1);
        let bottom_right = self.center + Vec2::new(self.radius as i32 + 1, self.radius as i32 + 1);

        Square::new(top_left, bottom_right).coords()
            .filter(|c| c.manhattan_distance_from(self.center) <= self.radius as usize)
    }

    fn from_center_and_point_on_boundary(center: Vec2, point_on_boundary: Vec2) -> Self {
        let radius = center.manhattan_distance_from(point_on_boundary);
        Diamond { center, radius: radius.try_into().unwrap() }
    }
}

impl AocSolution for Day15 {
    type Input = Vec<(Sensor, Beacon)>;
    fn process_input(input: &str) -> Self::Input {
        input.lines()
            .map(|l| {
                let [sensor_x, sensor_y, beacon_x, beacon_y] = inputs::n_numbers(l);
                let sensor = Vec2::new(sensor_x, sensor_y);
                let beacon = Vec2::new(beacon_x, beacon_y);
                (sensor, beacon)
            })
            .collect()
    }

    const PART1_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART1_SOLUTION: Solution = Solution::Unsolved;
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        let beacons: HashSet<Beacon> = input.iter()
            .map(|&(_, beacon)| beacon)
            .collect();

        input.iter()
            .flat_map(|&(sensor, beacon)| {
                let radius = sensor.manhattan_distance_from(beacon);
                let top_left = sensor - Vec2::new(radius as i32 - 1, radius as i32 - 1);
                let bottom_right = sensor + Vec2::new(radius as i32 + 1, radius as i32 + 1);
                Square::new(top_left, bottom_right).coords()
                    .filter(|c| c.manhattan_distance_from(sensor) <= radius as usize)
                // Diamond { center: sensor, radius: radius.try_into().unwrap() }.coords()
            })
            .unique()
            .filter(|c| c.y == 2000000 && !beacons.contains(c))
            .count()
    }

    const PART2_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART2_SOLUTION: Solution = Solution::Unsolved;
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}
