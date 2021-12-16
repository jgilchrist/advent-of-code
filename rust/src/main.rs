use anyhow::Result;

mod aoc;
mod runner;
mod utils;

pub use aoc::{AocSolution, AocYear, Unsolved};

mod y2021;

fn main() -> Result<()> {
    runner::init();
    runner::run_year::<y2021::Y2021, 2021>()?;
    Ok(())
}
