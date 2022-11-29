#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::approx_constant)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::too_many_lines)]

use std::env;

use anyhow::{bail, Result};

mod progress;
mod runner;

fn main() -> Result<()> {
    runner::init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => progress::print_progress(),
        2 | 3 => {
            let year_str = &args[1];
            let year = year_str.parse::<u32>().unwrap();

            match args.len() {
                2 => match year {
                    2015 => runner::run_year::<y2015::Y2015, 2015>()?,
                    2016 => runner::run_year::<y2016::Y2016, 2016>()?,
                    2017 => runner::run_year::<y2017::Y2017, 2017>()?,
                    2018 => runner::run_year::<y2018::Y2018, 2018>()?,
                    2019 => runner::run_year::<y2019::Y2019, 2019>()?,
                    2020 => runner::run_year::<y2020::Y2020, 2020>()?,
                    2021 => runner::run_year::<y2021::Y2021, 2021>()?,
                    _ => bail!("invalid year"),
                },
                3 => {
                    let day_str = &args[2];
                    let day_n = day_str.parse::<u32>()?;

                    match year {
                        2015 => runner::run_year_solution::<y2015::Y2015, 2015>(day_n)?,
                        2016 => runner::run_year_solution::<y2016::Y2016, 2016>(day_n)?,
                        2017 => runner::run_year_solution::<y2017::Y2017, 2017>(day_n)?,
                        2018 => runner::run_year_solution::<y2018::Y2018, 2018>(day_n)?,
                        2019 => runner::run_year_solution::<y2019::Y2019, 2019>(day_n)?,
                        2020 => runner::run_year_solution::<y2020::Y2020, 2020>(day_n)?,
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
