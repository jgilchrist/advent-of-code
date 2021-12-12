use anyhow::Result;
use std::{fmt::Display, path::PathBuf};

mod y2021;

pub trait AocSolution {
    type Input;
    type Output: Display + Eq;

    const YEAR: u32;
    const DAY: u32;

    fn process_input(&self, input: &str) -> Self::Input;

    const PART1_SOLUTION: Option<Self::Output>;
    fn part1(&self, i: &Self::Input) -> Self::Output;

    const PART2_SOLUTION: Option<Self::Output>;
    fn part2(&self, i: &Self::Input) -> Self::Output;
}

fn run_solution<TSln: AocSolution>(solution: TSln) -> Result<()> {
    let path: PathBuf = [
        "src",
        &format!("y{}", TSln::YEAR),
        &format!("d{:0>2}.in", TSln::DAY),
    ]
    .iter()
    .collect();

    let input = std::fs::read_to_string(path)?;
    let processed_input = solution.process_input(&input);

    let part1_solution = solution.part1(&processed_input);
    println!("{}", part1_solution);

    if let Some(expected_p1_solution) = TSln::PART1_SOLUTION {
        if part1_solution == expected_p1_solution {
            println!("Matches")
        } else {
            println!("Does Not Match")
        }
    }

    let part2_solution = solution.part2(&processed_input);
    println!("{}", part2_solution);

    if let Some(expected_p2_solution) = TSln::PART2_SOLUTION {
        if part2_solution == expected_p2_solution {
            println!("Matches")
        } else {
            println!("Does Not Match")
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    run_solution(y2021::Day01)?;
    Ok(())
}
