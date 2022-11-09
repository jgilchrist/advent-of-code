#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::approx_constant)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::integer_division)]
#![allow(clippy::implicit_return)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::todo)]
#![allow(clippy::shadow_reuse)]
#![allow(clippy::unreachable)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::std_instead_of_core)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::panic_in_result_fn)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::use_debug)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::string_add)]
#![allow(clippy::string_add_assign)]
#![allow(clippy::string_slice)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::exit)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::self_named_module_files)]
#![allow(clippy::single_char_lifetime_names)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::too_many_lines)]

use std::env;

use anyhow::{bail, Result};

mod aoc;
mod progress;
mod runner;
mod utils;

pub use aoc::{AocSolution, AocYear, Unsolved};

mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;

fn main() -> Result<()> {
    runner::init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => progress::print_progress(),
        2 | 3 => {
            let year_str = args.get(1).unwrap();
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
                    let day_str = args.get(2).unwrap();
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
