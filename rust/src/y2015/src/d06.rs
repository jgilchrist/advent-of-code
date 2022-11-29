use aoc::{AocSolution, Solution};
use utils::{
    geometry::Square,
    inputs::{transform_lines_by_regex, Captures, TransformRegexes},
    vecs::Vec2,
};

pub struct Day06;

#[derive(Debug)]
pub enum Command {
    TurnOff,
    TurnOn,
    Toggle,
}

pub trait CapturesExtensions {
    fn get_square(&self) -> Square;
}

impl CapturesExtensions for Captures<'_> {
    fn get_square(&self) -> Square {
        Square::new(
            Vec2::new(self.get_i32(1), self.get_i32(2)),
            Vec2::new(self.get_i32(3), self.get_i32(4)),
        )
    }
}

impl AocSolution for Day06 {
    fn get_input() -> &'static str {
        include_str!("d06.in")
    }

    type Input = Vec<(Square, Command)>;
    fn process_input(input: &str) -> Self::Input {
        let regexes: TransformRegexes<(Square, Command)> = vec![
            (
                r#"turn on (\d+),(\d+) through (\d+),(\d+)"#,
                Box::new(move |c| (c.get_square(), Command::TurnOn)),
            ),
            (
                r#"turn off (\d+),(\d+) through (\d+),(\d+)"#,
                Box::new(move |c| (c.get_square(), Command::TurnOff)),
            ),
            (
                r#"toggle (\d+),(\d+) through (\d+),(\d+)"#,
                Box::new(move |c| (c.get_square(), Command::Toggle)),
            ),
        ];

        transform_lines_by_regex(input, regexes)
    }

    type Part1Output = usize;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(400410);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let mut points = Vec::new();

        for x in 0..1000 {
            for y in 0..1000 {
                let pt = Vec2::new(x, y);
                let mut lit = false;

                for (square, command) in input {
                    if square.contains(pt) {
                        lit = match command {
                            Command::TurnOn => true,
                            Command::TurnOff => false,
                            Command::Toggle => !lit,
                        }
                    }
                }

                if lit {
                    points.push(pt);
                }
            }
        }

        points.len()
    }

    type Part2Output = isize;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(15343601);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let mut total_brightness = 0;

        for x in 0..1000 {
            for y in 0..1000 {
                let pt = Vec2::new(x, y);
                let mut brightness = 0;

                for (square, command) in input {
                    if square.contains(pt) {
                        brightness = match command {
                            Command::TurnOn => brightness + 1,
                            Command::TurnOff => (brightness - 1).max(0),
                            Command::Toggle => brightness + 2,
                        }
                    }
                }

                total_brightness += brightness;
            }
        }

        total_brightness
    }
}
