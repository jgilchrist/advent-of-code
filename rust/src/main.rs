use anyhow::Result;
use std::{fmt::Display, path::PathBuf};

mod y2021;

pub trait AocSolution {
    type Input;
    type Output: Display;

    const YEAR: u32;
    const DAY: u32;

    fn process_input(&self, input: &str) -> Self::Input;
    fn part1(&self, i: &Self::Input) -> Self::Output;
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

    let part_1_solution = solution.part1(&processed_input);
    println!("{}", part_1_solution);

    let part_2_solution = solution.part2(&processed_input);
    println!("{}", part_2_solution);

    Ok(())
}

fn main() -> Result<()> {
    run_solution(y2021::Day01)?;
    Ok(())
}
