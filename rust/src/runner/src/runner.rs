#![allow(clippy::print_stdout)]

use aoc::{AocSolution, AocYear, SolutionStatus};
use std::time::{Duration, Instant};

use console::style;

pub fn init() {
    // Ensure the cursor is always visible on exit, even if it was hidden
    let _ctrlc_handle = ctrlc::set_handler(move || {
        let term = console::Term::stderr();
        let _show_cursor_handle = term.show_cursor();
        std::process::exit(1);
    });
}

pub fn run_solution<TSln: AocSolution, const NDAY: u32, const NYEAR: u32>() {
    println!(
        "{}{} Day {:0>2}",
        style("=").red().bold(),
        style("=").green().bold(),
        style(NDAY).yellow().bold()
    );

    let start_time = Instant::now();
    let input = get_input(NYEAR, NDAY);
    let processed_input = TSln::process_input(&input);
    let process_input_duration = start_time.elapsed();

    if process_input_duration.as_secs() >= 1 {
        println!(
            "{}: {:?}",
            style("Input").black().bold(),
            style(process_input_duration).yellow()
        );
    }

    match TSln::PART1_STATUS {
        SolutionStatus::Solved | SolutionStatus::Wip => {
            print!("{}: ", style("1").red().bold());

            let p1_started_timestamp = Instant::now();
            let part1_solution = TSln::part1(&processed_input).into();
            print!("{part1_solution}");

            let p1_checked = match TSln::PART1_STATUS {
                SolutionStatus::Solved => {
                    if TSln::PART1_SOLUTION == part1_solution {
                        SolutionCheckStatus::Correct
                    } else {
                        SolutionCheckStatus::Incorrect
                    }
                }
                SolutionStatus::SolvedInPython | SolutionStatus::Wip => {
                    SolutionCheckStatus::Unknown
                }
                SolutionStatus::Unsolved => unreachable!(),
            };

            print!(
                " {}{}, {}{}",
                style("(").black().bold(),
                p1_checked,
                format_duration(p1_started_timestamp.elapsed()),
                style(")").black().bold(),
            );
        }
        SolutionStatus::SolvedInPython | SolutionStatus::Unsolved => {
            print!(
                "{}: {}",
                style("1").black().bold(),
                style("Unsolved").black().bold()
            );
        }
    }

    println!();

    match TSln::PART2_STATUS {
        SolutionStatus::Solved | SolutionStatus::Wip => {
            print!("{}: ", style("2").red().bold());

            let p2_started_timestamp = Instant::now();
            let part2_solution = TSln::part2(&processed_input).into();
            print!("{part2_solution}");

            let p2_checked = match TSln::PART2_STATUS {
                SolutionStatus::Solved => {
                    if TSln::PART2_SOLUTION == part2_solution {
                        SolutionCheckStatus::Correct
                    } else {
                        SolutionCheckStatus::Incorrect
                    }
                }
                SolutionStatus::SolvedInPython | SolutionStatus::Wip => {
                    SolutionCheckStatus::Unknown
                }
                SolutionStatus::Unsolved => unreachable!(),
            };

            print!(
                " {}{}, {}{}",
                style("(").black().bold(),
                p2_checked,
                format_duration(p2_started_timestamp.elapsed()),
                style(")").black().bold(),
            );
        }
        SolutionStatus::SolvedInPython | SolutionStatus::Unsolved => {
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
    let file_path_within_directory_structure = format!("y{year}/src/d{day:0>2}.in");
    let filename = format!("{manifest_dir}/../{file_path_within_directory_structure}");

    std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("No input file: {file_path_within_directory_structure}"))
}

pub fn run_year<TYear: AocYear, const NYEAR: u32>() {
    run_solution::<TYear::D01, 1, NYEAR>();
    run_solution::<TYear::D02, 2, NYEAR>();
    run_solution::<TYear::D03, 3, NYEAR>();
    run_solution::<TYear::D04, 4, NYEAR>();
    run_solution::<TYear::D05, 5, NYEAR>();
    run_solution::<TYear::D06, 6, NYEAR>();
    run_solution::<TYear::D07, 7, NYEAR>();
    run_solution::<TYear::D08, 8, NYEAR>();
    run_solution::<TYear::D09, 9, NYEAR>();
    run_solution::<TYear::D10, 10, NYEAR>();
    run_solution::<TYear::D11, 11, NYEAR>();
    run_solution::<TYear::D12, 12, NYEAR>();
    run_solution::<TYear::D13, 13, NYEAR>();
    run_solution::<TYear::D14, 14, NYEAR>();
    run_solution::<TYear::D15, 15, NYEAR>();
    run_solution::<TYear::D16, 16, NYEAR>();
    run_solution::<TYear::D17, 17, NYEAR>();
    run_solution::<TYear::D18, 18, NYEAR>();
    run_solution::<TYear::D19, 19, NYEAR>();
    run_solution::<TYear::D20, 20, NYEAR>();
    run_solution::<TYear::D21, 21, NYEAR>();
    run_solution::<TYear::D22, 22, NYEAR>();
    run_solution::<TYear::D23, 23, NYEAR>();
    run_solution::<TYear::D24, 24, NYEAR>();
    run_solution::<TYear::D25, 25, NYEAR>();
}

pub fn run_year_solution<TYear: AocYear, const NYEAR: u32>(day: u32) {
    match day {
        1 => run_solution::<TYear::D01, 1, NYEAR>(),
        2 => run_solution::<TYear::D02, 2, NYEAR>(),
        3 => run_solution::<TYear::D03, 3, NYEAR>(),
        4 => run_solution::<TYear::D04, 4, NYEAR>(),
        5 => run_solution::<TYear::D05, 5, NYEAR>(),
        6 => run_solution::<TYear::D06, 6, NYEAR>(),
        7 => run_solution::<TYear::D07, 7, NYEAR>(),
        8 => run_solution::<TYear::D08, 8, NYEAR>(),
        9 => run_solution::<TYear::D09, 9, NYEAR>(),
        10 => run_solution::<TYear::D10, 10, NYEAR>(),
        11 => run_solution::<TYear::D11, 11, NYEAR>(),
        12 => run_solution::<TYear::D12, 12, NYEAR>(),
        13 => run_solution::<TYear::D13, 13, NYEAR>(),
        14 => run_solution::<TYear::D14, 14, NYEAR>(),
        15 => run_solution::<TYear::D15, 15, NYEAR>(),
        16 => run_solution::<TYear::D16, 16, NYEAR>(),
        17 => run_solution::<TYear::D17, 17, NYEAR>(),
        18 => run_solution::<TYear::D18, 18, NYEAR>(),
        19 => run_solution::<TYear::D19, 19, NYEAR>(),
        20 => run_solution::<TYear::D20, 20, NYEAR>(),
        21 => run_solution::<TYear::D21, 21, NYEAR>(),
        22 => run_solution::<TYear::D22, 22, NYEAR>(),
        23 => run_solution::<TYear::D23, 23, NYEAR>(),
        24 => run_solution::<TYear::D24, 24, NYEAR>(),
        25 => run_solution::<TYear::D25, 25, NYEAR>(),
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
    match d.as_millis() {
        ms if ms <= 1000 => format!("{:?}", style(d).green()),
        ms if ms > 1000 && ms <= 10000 => format!("{:?}", style(d).yellow()),
        ms if ms > 10000 => format!("{:?}", style(d).red()),
        _ => unreachable!(),
    }
}
