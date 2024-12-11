use aoc::prelude::*;
use utils::prelude::*;

pub struct Day02;

#[derive(Debug)]
pub struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    reveals: Vec<Cubes>,
}

impl AocSolution for Day02 {
    type Input = Vec<Game>;
    fn process_input(input: &str) -> Self::Input {
        let lines = inputs::lines(input);

        lines
            .iter()
            .map(|l| {
                let (game_prefix, l) = l.split_once(':').unwrap();

                let id = game_prefix
                    .strip_prefix("Game ")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let reveals = l
                    .split(';')
                    .map(|g| {
                        let red_matches = inputs::match_in_string(g, r"(\d+) red");
                        let blue_matches = inputs::match_in_string(g, r"(\d+) blue");
                        let green_matches = inputs::match_in_string(g, r"(\d+) green");

                        let red = red_matches.map_or(0, |mut m| m.next_u32());
                        let green = green_matches.map_or(0, |mut m| m.next_u32());
                        let blue = blue_matches.map_or(0, |mut m| m.next_u32());

                        Cubes { red, green, blue }
                    })
                    .collect_vec();

                Game { id, reveals }
            })
            .collect_vec()
    }

    const PART1_SOLUTION: SolutionStatus = solution(2776);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let cubes = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        input
            .iter()
            .filter(|g| {
                g.reveals
                    .iter()
                    .all(|r| r.red <= cubes.red && r.green <= cubes.green && r.blue <= cubes.blue)
            })
            .map(|g| g.id)
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(68638);
    fn part2(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .map(|g| Cubes {
                red: g.reveals.iter().map(|r| r.red).max().unwrap(),
                green: g.reveals.iter().map(|r| r.green).max().unwrap(),
                blue: g.reveals.iter().map(|r| r.blue).max().unwrap(),
            })
            .map(|g| g.red * g.green * g.blue)
            .sum::<u32>()
    }
}
