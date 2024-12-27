use aoc::prelude::*;
use utils::geometry::d2::vecs::Vec2;
use utils::prelude::*;

pub struct Day14;

#[derive(Clone)]
pub struct Robot {
    pub position: Vec2,
    pub velocity: Vec2,
}

fn add_mod(v1: Vec2, v2: Vec2, vmod: Vec2) -> Vec2 {
    let mut xnew = v1.x + v2.x;
    let mut ynew = v1.y + v2.y;

    if xnew < 0 {
        xnew += vmod.x;
    } else {
        xnew %= vmod.x;
    }

    if ynew < 0 {
        ynew += vmod.y;
    } else {
        ynew %= vmod.y;
    }

    Vec2::new(xnew, ynew)
}

fn variance(ns: &[f64]) -> f64 {
    let mean = mean(ns);

    ns.iter().map(|n| (n - mean).abs().powi(2)).sum::<f64>() / ns.len() as f64
}

fn mean(ns: &[f64]) -> f64 {
    ns.iter().sum::<f64>() / ns.len() as f64
}

impl AocSolution for Day14 {
    type Input = Vec<Robot>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let [px, py, vx, vy] = inputs::n_numbers(l);

                Robot {
                    position: Vec2::new(px, py),
                    velocity: Vec2::new(vx, vy),
                }
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(225521010);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut robots = input.clone();
        let grid_size = Vec2::new(101, 103);

        for _ in 0..100 {
            for robot in &mut robots {
                robot.position = add_mod(robot.position, robot.velocity, grid_size);
            }
        }

        let x_midpoint = grid_size.x / 2;
        let y_midpoint = grid_size.y / 2;

        let north_west_robots = robots
            .iter()
            .filter(|r| r.position.x < x_midpoint && r.position.y < y_midpoint)
            .count();

        let north_east_robots = robots
            .iter()
            .filter(|r| r.position.x > x_midpoint && r.position.y < y_midpoint)
            .count();

        let south_west_robots = robots
            .iter()
            .filter(|r| r.position.x < x_midpoint && r.position.y > y_midpoint)
            .count();

        let south_east_robots = robots
            .iter()
            .filter(|r| r.position.x > x_midpoint && r.position.y > y_midpoint)
            .count();

        north_west_robots * north_east_robots * south_west_robots * south_east_robots
    }

    const PART2_SOLUTION: SolutionStatus = solution(7774);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut robots = input.clone();
        let grid_size = Vec2::new(101, 103);

        let mut step = 0;
        loop {
            step += 1;

            for robot in &mut robots {
                robot.position = add_mod(robot.position, robot.velocity, grid_size);
            }

            // Assumption: The image is going to have relatively low variance, since
            // the robots are going to have to be relatively close together to form
            // an image.
            let x_variance = variance(
                &robots
                    .iter()
                    .map(|r| f64::from(r.position.x))
                    .collect::<Vec<_>>(),
            );

            let y_variance = variance(
                &robots
                    .iter()
                    .map(|r| f64::from(r.position.y))
                    .collect::<Vec<_>>(),
            );

            // Values chosen manually by printing variances and seeing the average values.
            if x_variance < 400.0 && y_variance < 400.0 {
                return step;
            }
        }
    }
}
