use anyhow::Result;
use std::{fmt::Display, path::PathBuf};

mod y2021;

pub trait AocYear {
    const YEAR: u32;

    type D01: AocSolution;
    type D02: AocSolution;
    type D03: AocSolution;
    type D04: AocSolution;
    type D05: AocSolution;
    type D06: AocSolution;
    type D07: AocSolution;
    type D08: AocSolution;
    type D09: AocSolution;
    type D10: AocSolution;
    type D11: AocSolution;
    type D12: AocSolution;
    type D13: AocSolution;
    type D14: AocSolution;
    type D15: AocSolution;
    type D16: AocSolution;
    type D17: AocSolution;
    type D18: AocSolution;
    type D19: AocSolution;
    type D20: AocSolution;
    type D21: AocSolution;
    type D22: AocSolution;
    type D23: AocSolution;
    type D24: AocSolution;
    type D25: AocSolution;
}

pub trait AocSolution {
    type Input;
    type Output: Display + Eq;

    const DAY: u32;
    const SOLVED: bool = true;

    fn process_input(input: &str) -> Self::Input;

    const PART1_SOLUTION: Option<Self::Output>;
    fn part1(i: &Self::Input) -> Self::Output;

    const PART2_SOLUTION: Option<Self::Output>;
    fn part2(i: &Self::Input) -> Self::Output;
}

pub struct Unsolved;

impl AocSolution for Unsolved {
    type Input = ();
    type Output = &'static str;

    const DAY: u32 = 0;
    const SOLVED: bool = false;

    fn process_input(_: &str) -> Self::Input {
        unimplemented!()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(_: &Self::Input) -> Self::Output {
        unimplemented!()
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(_: &Self::Input) -> Self::Output {
        unimplemented!()
    }
}

fn run_year<TYear: AocYear>() -> Result<()> {
    run_solution::<TYear, TYear::D01>()?;
    run_solution::<TYear, TYear::D02>()?;
    run_solution::<TYear, TYear::D03>()?;
    run_solution::<TYear, TYear::D04>()?;
    run_solution::<TYear, TYear::D05>()?;
    run_solution::<TYear, TYear::D06>()?;
    run_solution::<TYear, TYear::D07>()?;
    run_solution::<TYear, TYear::D08>()?;
    run_solution::<TYear, TYear::D09>()?;
    run_solution::<TYear, TYear::D10>()?;
    run_solution::<TYear, TYear::D11>()?;
    run_solution::<TYear, TYear::D12>()?;
    run_solution::<TYear, TYear::D13>()?;
    run_solution::<TYear, TYear::D14>()?;
    run_solution::<TYear, TYear::D15>()?;
    run_solution::<TYear, TYear::D16>()?;
    run_solution::<TYear, TYear::D17>()?;
    run_solution::<TYear, TYear::D18>()?;
    run_solution::<TYear, TYear::D19>()?;
    run_solution::<TYear, TYear::D20>()?;
    run_solution::<TYear, TYear::D21>()?;
    run_solution::<TYear, TYear::D22>()?;
    run_solution::<TYear, TYear::D23>()?;
    run_solution::<TYear, TYear::D24>()?;
    run_solution::<TYear, TYear::D25>()?;
    Ok(())
}

fn run_solution<TYear: AocYear, TSln: AocSolution>() -> Result<()> {
    if !TSln::SOLVED {
        return Ok(());
    }

    let path: PathBuf = [
        "src",
        &format!("y{}", TYear::YEAR),
        &format!("d{:0>2}.in", TSln::DAY),
    ]
    .iter()
    .collect();

    let input = std::fs::read_to_string(path)?;
    let processed_input = TSln::process_input(&input);

    let part1_solution = TSln::part1(&processed_input);
    println!("{}", part1_solution);

    if let Some(expected_p1_solution) = TSln::PART1_SOLUTION {
        if part1_solution == expected_p1_solution {
            println!("Matches")
        } else {
            println!("Does Not Match")
        }
    }

    let part2_solution = TSln::part2(&processed_input);
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
    run_year::<y2021::Y2021>()?;
    Ok(())
}
