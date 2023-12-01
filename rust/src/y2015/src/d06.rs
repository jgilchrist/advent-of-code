use prelude::*;
use utils::geometry::d2::shapes::Square;

pub struct Day06;

#[derive(Debug)]
pub enum Command {
    TurnOff,
    TurnOn,
    Toggle,
}

pub trait CapturesExtensions {
    fn next_square(&mut self) -> Square;
}

impl CapturesExtensions for inputs::Captures<'_> {
    fn next_square(&mut self) -> Square {
        Square::new(
            (self.next_i32(), self.next_i32()),
            (self.next_i32(), self.next_i32()),
        )
    }
}

impl AocSolution for Day06 {
    type Input = Vec<(Square, Command)>;
    fn process_input(input: &str) -> Self::Input {
        inputs::regexes(
            input,
            vec![
                (
                    r#"turn on (\d+),(\d+) through (\d+),(\d+)"#,
                    Box::new(move |mut c| (c.next_square(), Command::TurnOn)),
                ),
                (
                    r#"turn off (\d+),(\d+) through (\d+),(\d+)"#,
                    Box::new(move |mut c| (c.next_square(), Command::TurnOff)),
                ),
                (
                    r#"toggle (\d+),(\d+) through (\d+),(\d+)"#,
                    Box::new(move |mut c| (c.next_square(), Command::Toggle)),
                ),
            ],
        )
    }

    const PART1_SOLUTION: Solution = solution(400410);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut points = Vec::new();

        for x in 0..1000 {
            for y in 0..1000 {
                let pt = (x, y);
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

    const PART2_SOLUTION: Solution = solution(15343601);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut total_brightness = 0;

        for x in 0..1000 {
            for y in 0..1000 {
                let pt = (x, y);
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
