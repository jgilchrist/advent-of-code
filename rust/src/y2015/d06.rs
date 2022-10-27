use crate::{
    aoc::Solution,
    utils::{geometry::Square, vecs::Vec2},
    AocSolution,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0},
    combinator::eof,
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
};

pub struct Day06;

#[derive(Debug)]
pub enum Command {
    TurnOff,
    TurnOn,
    Toggle,
}

fn vec2(input: &str) -> nom::IResult<&str, Vec2> {
    let (input, (x, y)) = separated_pair(
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
    )(input)?;
    Ok((input, Vec2::new(x, y)))
}

fn turn_on_command(input: &str) -> nom::IResult<&str, (Square, Command)> {
    let (input, (_, top_left, _, bottom_right)) = tuple((
        preceded(space0, tag("turn on")),
        preceded(space0, vec2),
        preceded(space0, tag("through")),
        preceded(space0, vec2),
    ))(input)?;

    let square = Square::new(top_left, bottom_right);
    let command = (square, Command::TurnOn);

    Ok((input, command))
}

fn turn_off_command(input: &str) -> nom::IResult<&str, (Square, Command)> {
    let (input, (_, top_left, _, bottom_right)) = tuple((
        preceded(space0, tag("turn off")),
        preceded(space0, vec2),
        preceded(space0, tag("through")),
        preceded(space0, vec2),
    ))(input)?;

    let square = Square::new(top_left, bottom_right);
    let command = (square, Command::TurnOff);

    Ok((input, command))
}

fn toggle_command(input: &str) -> nom::IResult<&str, (Square, Command)> {
    let (input, (_, top_left, _, bottom_right)) = tuple((
        preceded(space0, tag("toggle")),
        preceded(space0, vec2),
        preceded(space0, tag("through")),
        preceded(space0, vec2),
    ))(input)?;

    let square = Square::new(top_left, bottom_right);
    let command = (square, Command::Toggle);

    Ok((input, command))
}

fn command(input: &str) -> nom::IResult<&str, (Square, Command)> {
    alt((turn_on_command, turn_off_command, toggle_command))(input)
}

fn commands(input: &str) -> nom::IResult<&str, Vec<(Square, Command)>> {
    terminated(many1(terminated(command, newline)), eof)(input)
}

fn parse(input: &str) -> Vec<(Square, Command)> {
    let (_, command_list) = commands(input).unwrap();
    command_list
}

impl AocSolution for Day06 {
    fn get_input() -> &'static str {
        include_str!("d06.in")
    }

    type Input = Vec<(Square, Command)>;
    fn process_input(input: &str) -> Self::Input {
        parse(input)
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

                total_brightness += brightness
            }
        }

        total_brightness
    }
}
