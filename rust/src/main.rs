use std::env;

use anyhow::{bail, Result};

mod aoc;
mod runner;
mod utils;

pub use aoc::{AocSolution, AocYear, Unsolved};

mod y2021;

fn main() -> Result<()> {
    runner::init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => runner::run_year::<y2021::Y2021, 2021>()?,
        2 | 3 => {
            let year_str = args.get(1).unwrap();
            let year = year_str.parse::<u32>().unwrap();

            match args.len() {
                2 => match year {
                    2021 => runner::run_year::<y2021::Y2021, 2021>()?,
                    _ => bail!("invalid year"),
                },
                3 => {
                    let day_str = args.get(2).unwrap();
                    let day_n = day_str.parse::<u32>()?;

                    match year {
                        2021 => runner::run_year_solution::<y2021::Y2021, 2021>(day_n)?,
                        _ => bail!("invalid year"),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => bail!("usage: [year] [day?]"),
    }

    Ok(())
}
