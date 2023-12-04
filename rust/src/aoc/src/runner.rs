#![allow(clippy::print_stdout)]

use crate::{AocSolution, AocYear, SolutionStatus, ToSolution};
use anyhow::{bail, Result};
use std::env;
use std::time::{Duration, Instant};

use console::style;

pub fn main<TYear: AocYear>() -> Result<()> {
    init();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_year::<TYear>(),
        2 => {
            let day_str = &args[1];
            let day_n = day_str.parse::<u32>()?;

            run_year_solution::<TYear>(day_n);
        }
        _ => bail!("usage: [day?]"),
    }

    Ok(())
}

fn init() {
    // Ensure the cursor is always visible on exit, even if it was hidden
    let _ctrlc_handle = ctrlc::set_handler(move || {
        let term = console::Term::stderr();
        let _show_cursor_handle = term.show_cursor();
        std::process::exit(1);
    });
}

fn run_solution<TYear: AocYear, TSln: AocSolution, const NDAY: u32>() {
    if !TSln::IS_SOLVED {
        return;
    }

    println!(
        "{}{} Day {:0>2}",
        style("=").red().bold(),
        style("=").green().bold(),
        style(NDAY).yellow().bold()
    );

    let start_time = Instant::now();
    let input = get_input(TYear::YEAR, NDAY);
    let processed_input = TSln::process_input(&input);
    let process_input_duration = start_time.elapsed();

    if process_input_duration.as_secs() >= 1 {
        println!(
            "{}: {:?}",
            style("Input").black().bold(),
            style(process_input_duration).yellow()
        );
    }

    match TSln::PART1_SOLUTION {
        SolutionStatus::Solved(_) | SolutionStatus::Wip => {
            print!("{}: ", style("1").red().bold());

            let p1_started_timestamp = Instant::now();
            let part1_solution = TSln::part1(&processed_input).to_solution();
            print!("{part1_solution}");

            let p1_checked = match TSln::PART1_SOLUTION {
                SolutionStatus::Solved(sln) => {
                    if sln == part1_solution {
                        SolutionCheckStatus::Correct
                    } else {
                        SolutionCheckStatus::Incorrect
                    }
                }
                SolutionStatus::Wip => SolutionCheckStatus::Unknown,
                SolutionStatus::SolvedInPython(_) | SolutionStatus::Unsolved => unreachable!(),
            };

            print!(
                " {}{}, {}{}",
                style("(").black().bold(),
                p1_checked,
                format_duration(p1_started_timestamp.elapsed()),
                style(")").black().bold(),
            );
        }
        SolutionStatus::SolvedInPython(_) | SolutionStatus::Unsolved => {
            print!(
                "{}: {}",
                style("1").black().bold(),
                style("Unsolved").black().bold()
            );
        }
    }

    println!();

    match TSln::PART2_SOLUTION {
        SolutionStatus::Solved(_) | SolutionStatus::Wip => {
            print!("{}: ", style("2").red().bold());

            let p2_started_timestamp = Instant::now();
            let part2_solution = TSln::part2(&processed_input).to_solution();
            print!("{part2_solution}");

            let p2_checked = match TSln::PART2_SOLUTION {
                SolutionStatus::Solved(sln) => {
                    if sln == part2_solution {
                        SolutionCheckStatus::Correct
                    } else {
                        SolutionCheckStatus::Incorrect
                    }
                }
                SolutionStatus::Wip => SolutionCheckStatus::Unknown,
                SolutionStatus::SolvedInPython(_) | SolutionStatus::Unsolved => unreachable!(),
            };

            print!(
                " {}{}, {}{}",
                style("(").black().bold(),
                p2_checked,
                format_duration(p2_started_timestamp.elapsed()),
                style(")").black().bold(),
            );
        }
        SolutionStatus::SolvedInPython(_) | SolutionStatus::Unsolved => {
            print!(
                "{}: {}",
                style("2").black().bold(),
                style("Unsolved").black().bold()
            );
        }
    }

    println!();
}

fn get_input(year: u32, day: u32) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path_within_directory_structure = format!("{year}/{day:0>2}.in");
    let filename = format!("{manifest_dir}/../../../inputs/{file_path_within_directory_structure}");

    std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("No input file: {file_path_within_directory_structure}"))
}

fn run_year<TYear: AocYear>() {
    run_solution::<TYear, TYear::D01, 1>();
    run_solution::<TYear, TYear::D02, 2>();
    run_solution::<TYear, TYear::D03, 3>();
    run_solution::<TYear, TYear::D04, 4>();
    run_solution::<TYear, TYear::D05, 5>();
    run_solution::<TYear, TYear::D06, 6>();
    run_solution::<TYear, TYear::D07, 7>();
    run_solution::<TYear, TYear::D08, 8>();
    run_solution::<TYear, TYear::D09, 9>();
    run_solution::<TYear, TYear::D10, 10>();
    run_solution::<TYear, TYear::D11, 11>();
    run_solution::<TYear, TYear::D12, 12>();
    run_solution::<TYear, TYear::D13, 13>();
    run_solution::<TYear, TYear::D14, 14>();
    run_solution::<TYear, TYear::D15, 15>();
    run_solution::<TYear, TYear::D16, 16>();
    run_solution::<TYear, TYear::D17, 17>();
    run_solution::<TYear, TYear::D18, 18>();
    run_solution::<TYear, TYear::D19, 19>();
    run_solution::<TYear, TYear::D20, 20>();
    run_solution::<TYear, TYear::D21, 21>();
    run_solution::<TYear, TYear::D22, 22>();
    run_solution::<TYear, TYear::D23, 23>();
    run_solution::<TYear, TYear::D24, 24>();
    run_solution::<TYear, TYear::D25, 25>();
}

fn run_year_solution<TYear: AocYear>(day: u32) {
    match day {
        1 => run_solution::<TYear, TYear::D01, 1>(),
        2 => run_solution::<TYear, TYear::D02, 2>(),
        3 => run_solution::<TYear, TYear::D03, 3>(),
        4 => run_solution::<TYear, TYear::D04, 4>(),
        5 => run_solution::<TYear, TYear::D05, 5>(),
        6 => run_solution::<TYear, TYear::D06, 6>(),
        7 => run_solution::<TYear, TYear::D07, 7>(),
        8 => run_solution::<TYear, TYear::D08, 8>(),
        9 => run_solution::<TYear, TYear::D09, 9>(),
        10 => run_solution::<TYear, TYear::D10, 10>(),
        11 => run_solution::<TYear, TYear::D11, 11>(),
        12 => run_solution::<TYear, TYear::D12, 12>(),
        13 => run_solution::<TYear, TYear::D13, 13>(),
        14 => run_solution::<TYear, TYear::D14, 14>(),
        15 => run_solution::<TYear, TYear::D15, 15>(),
        16 => run_solution::<TYear, TYear::D16, 16>(),
        17 => run_solution::<TYear, TYear::D17, 17>(),
        18 => run_solution::<TYear, TYear::D18, 18>(),
        19 => run_solution::<TYear, TYear::D19, 19>(),
        20 => run_solution::<TYear, TYear::D20, 20>(),
        21 => run_solution::<TYear, TYear::D21, 21>(),
        22 => run_solution::<TYear, TYear::D22, 22>(),
        23 => run_solution::<TYear, TYear::D23, 23>(),
        24 => run_solution::<TYear, TYear::D24, 24>(),
        25 => run_solution::<TYear, TYear::D25, 25>(),
        _ => panic!("invalid day"),
    }
}

enum SolutionCheckStatus {
    Unknown,
    Incorrect,
    Correct,
}

impl std::fmt::Display for SolutionCheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SolutionCheckStatus::*;
        write!(
            f,
            "{}",
            match *self {
                Correct => style("+").green(),
                Incorrect => style("x").red(),
                Unknown => style("?").black().bold(),
            }
        )
    }
}

fn format_duration(d: Duration) -> String {
    format!(
        "{:.1?}",
        match d.as_millis() {
            ms if ms <= 1000 => style(d).green(),
            ms if ms > 1000 && ms <= 10000 => style(d).yellow(),
            ms if ms > 10000 => style(d).red(),
            _ => unreachable!(),
        }
    )
}
