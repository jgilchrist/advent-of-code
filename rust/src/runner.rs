use anyhow::{bail, Result};
use std::time::{Duration, Instant};

use console::style;

use crate::aoc::{AocSolution, AocYear};

pub fn init() {
    // Ensure the cursor is always visible on exit, even if it was hidden
    let _ = ctrlc::set_handler(move || {
        let term = console::Term::stderr();
        let _ = term.show_cursor();
        std::process::exit(1);
    });
}

pub fn run_solution<TSln: AocSolution<NDAY>, const NDAY: u32, const NYEAR: u32>() -> Result<()> {
    if !TSln::SOLVED {
        println!(
            "{}{} Day {:0>2}",
            style("=").black().bold(),
            style("=").black().bold(),
            style(NDAY).yellow().bold()
        );

        return Ok(());
    }

    println!(
        "{}{} Day {:0>2}",
        style("=").red().bold(),
        style("=").green().bold(),
        style(NDAY).yellow().bold()
    );

    let start_time = Instant::now();
    let input = TSln::get_input();
    let processed_input = TSln::process_input(input);
    let process_input_duration = start_time.elapsed();

    if process_input_duration.as_secs() >= 1 {
        println!(
            "{}: {:?}",
            style("Input").black().bold(),
            style(process_input_duration).yellow()
        )
    }

    let test_data = TSln::tests();

    if !test_data.is_empty() {
        for (i, test) in test_data.iter().enumerate() {
            let (input, expected_part1_output, _) = test;
            let processed_input = TSln::process_input(input);

            if let Some(expected_part1_output) = expected_part1_output {
                let actual_part1_output = TSln::part1(&processed_input);
                let passed_part1_test = actual_part1_output == *expected_part1_output;

                if passed_part1_test {
                    println!(
                        "{} {}{} {}",
                        style("Test").black().bold(),
                        style(i + 1).black().bold(),
                        style(":").black().bold(),
                        style("+").green()
                    );
                } else {
                    println!(
                        "{} {} {}{} {}",
                        style(format!("Test {}:", i + 1)).red(),
                        style("expected").black().bold(),
                        style(expected_part1_output).green(),
                        style(", found").black().bold(),
                        style(actual_part1_output).red()
                    );

                    bail!("Failed test")
                }
            }
        }
    }

    print!("{}: ", style("1").red().bold(),);

    let p1_started_timestamp = Instant::now();
    let part1_solution = TSln::part1(&processed_input);
    print!("{}", part1_solution);

    let p1_checked = match TSln::PART1_SOLUTION {
        Some(expected) => {
            if expected == part1_solution {
                SolutionCheckStatus::Correct
            } else {
                SolutionCheckStatus::Incorrect
            }
        }
        None => SolutionCheckStatus::Unknown,
    };

    println!(
        " {}{}, {}{}",
        style("(").black().bold(),
        p1_checked,
        format_duration(p1_started_timestamp.elapsed()),
        style(")").black().bold(),
    );

    if !test_data.is_empty() {
        for (i, test) in test_data.iter().enumerate() {
            let (input, _, expected_part2_output) = test;
            let processed_input = TSln::process_input(input);

            if let Some(expected_part2_output) = expected_part2_output {
                let actual_part2_output = TSln::part2(&processed_input);
                let passed_part2_test = actual_part2_output == *expected_part2_output;

                if passed_part2_test {
                    println!(
                        "{} {}{} {}",
                        style("Test").black().bold(),
                        style(i + 1).black().bold(),
                        style(":").black().bold(),
                        style("+").green()
                    );
                } else {
                    println!(
                        "{} {} {}{} {}",
                        style(format!("Test {}:", i + 1)).red(),
                        style("expected").black().bold(),
                        style(expected_part2_output).green(),
                        style(", found").black().bold(),
                        style(actual_part2_output).red()
                    );

                    bail!("Failed test")
                }
            }
        }
    }

    print!("{}: ", style("2").green().bold(),);

    let p2_started_timestamp = Instant::now();
    let part2_solution = TSln::part2(&processed_input);
    print!("{}", part2_solution);

    let p2_checked = match TSln::PART2_SOLUTION {
        Some(expected) => {
            if expected == part2_solution {
                SolutionCheckStatus::Correct
            } else {
                SolutionCheckStatus::Incorrect
            }
        }
        None => SolutionCheckStatus::Unknown,
    };

    println!(
        " {}{}, {}{}",
        style("(").black().bold(),
        p2_checked,
        format_duration(p2_started_timestamp.elapsed()),
        style(")").black().bold(),
    );

    Ok(())
}

pub fn run_year<TYear: AocYear<NYEAR>, const NYEAR: u32>() -> Result<()> {
    run_solution::<TYear::D01, 1, NYEAR>()?;
    run_solution::<TYear::D02, 2, NYEAR>()?;
    run_solution::<TYear::D03, 3, NYEAR>()?;
    run_solution::<TYear::D04, 4, NYEAR>()?;
    run_solution::<TYear::D05, 5, NYEAR>()?;
    run_solution::<TYear::D06, 6, NYEAR>()?;
    run_solution::<TYear::D07, 7, NYEAR>()?;
    run_solution::<TYear::D08, 8, NYEAR>()?;
    run_solution::<TYear::D09, 9, NYEAR>()?;
    run_solution::<TYear::D10, 10, NYEAR>()?;
    run_solution::<TYear::D11, 11, NYEAR>()?;
    run_solution::<TYear::D12, 12, NYEAR>()?;
    run_solution::<TYear::D13, 13, NYEAR>()?;
    run_solution::<TYear::D14, 14, NYEAR>()?;
    run_solution::<TYear::D15, 15, NYEAR>()?;
    run_solution::<TYear::D16, 16, NYEAR>()?;
    run_solution::<TYear::D17, 17, NYEAR>()?;
    run_solution::<TYear::D18, 18, NYEAR>()?;
    run_solution::<TYear::D19, 19, NYEAR>()?;
    run_solution::<TYear::D20, 20, NYEAR>()?;
    run_solution::<TYear::D21, 21, NYEAR>()?;
    run_solution::<TYear::D22, 22, NYEAR>()?;
    run_solution::<TYear::D23, 23, NYEAR>()?;
    run_solution::<TYear::D24, 24, NYEAR>()?;
    run_solution::<TYear::D25, 25, NYEAR>()?;
    Ok(())
}

pub fn run_year_solution<TYear: AocYear<NYEAR>, const NYEAR: u32>(day: u32) -> Result<()> {
    match day {
        1 => run_solution::<TYear::D01, 1, NYEAR>()?,
        2 => run_solution::<TYear::D02, 2, NYEAR>()?,
        3 => run_solution::<TYear::D03, 3, NYEAR>()?,
        4 => run_solution::<TYear::D04, 4, NYEAR>()?,
        5 => run_solution::<TYear::D05, 5, NYEAR>()?,
        6 => run_solution::<TYear::D06, 6, NYEAR>()?,
        7 => run_solution::<TYear::D07, 7, NYEAR>()?,
        8 => run_solution::<TYear::D08, 8, NYEAR>()?,
        9 => run_solution::<TYear::D09, 9, NYEAR>()?,
        10 => run_solution::<TYear::D10, 10, NYEAR>()?,
        11 => run_solution::<TYear::D11, 11, NYEAR>()?,
        12 => run_solution::<TYear::D12, 12, NYEAR>()?,
        13 => run_solution::<TYear::D13, 13, NYEAR>()?,
        14 => run_solution::<TYear::D14, 14, NYEAR>()?,
        15 => run_solution::<TYear::D15, 15, NYEAR>()?,
        16 => run_solution::<TYear::D16, 16, NYEAR>()?,
        17 => run_solution::<TYear::D17, 17, NYEAR>()?,
        18 => run_solution::<TYear::D18, 18, NYEAR>()?,
        19 => run_solution::<TYear::D19, 19, NYEAR>()?,
        20 => run_solution::<TYear::D20, 20, NYEAR>()?,
        21 => run_solution::<TYear::D21, 21, NYEAR>()?,
        22 => run_solution::<TYear::D22, 22, NYEAR>()?,
        23 => run_solution::<TYear::D23, 23, NYEAR>()?,
        24 => run_solution::<TYear::D24, 24, NYEAR>()?,
        25 => run_solution::<TYear::D25, 25, NYEAR>()?,
        _ => bail!("invalid day"),
    }

    Ok(())
}

enum SolutionCheckStatus {
    Unknown,
    Incorrect,
    Correct,
}

impl std::fmt::Display for SolutionCheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                SolutionCheckStatus::Correct => style("+").green(),
                SolutionCheckStatus::Incorrect => style("x").red(),
                SolutionCheckStatus::Unknown => style("?").black().bold(),
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
