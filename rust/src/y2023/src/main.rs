#![feature(array_windows)]
#![feature(let_chains)]
#![allow(incomplete_features)]
#![allow(unreachable_patterns)]

use anyhow::{bail, Result};
use aoc::{AocYear, Unsolved};
use std::env;

pub struct Y2023;

mod d01;
mod d02;
mod d03;

impl AocYear for Y2023 {
    const YEAR: u32 = 2023;

    type D01 = d01::Day01;
    type D02 = d02::Day02;
    type D03 = d03::Day03;
    type D04 = Unsolved;
    type D05 = Unsolved;
    type D06 = Unsolved;
    type D07 = Unsolved;
    type D08 = Unsolved;
    type D09 = Unsolved;
    type D10 = Unsolved;
    type D11 = Unsolved;
    type D12 = Unsolved;
    type D13 = Unsolved;
    type D14 = Unsolved;
    type D15 = Unsolved;
    type D16 = Unsolved;
    type D17 = Unsolved;
    type D18 = Unsolved;
    type D19 = Unsolved;
    type D20 = Unsolved;
    type D21 = Unsolved;
    type D22 = Unsolved;
    type D23 = Unsolved;
    type D24 = Unsolved;
    type D25 = Unsolved;
}

fn main() -> Result<()> {
    aoc::init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => aoc::run_year::<Y2023>(),
        2 => {
            let day_str = &args[1];
            let day_n = day_str.parse::<u32>()?;

            aoc::run_year_solution::<Y2023>(day_n)
        }
        _ => bail!("usage: [day?]"),
    }

    Ok(())
}
