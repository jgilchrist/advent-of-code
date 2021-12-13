use anyhow::Result;
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
        return Ok(());
    }

    println!(
        "{}{} Day {:0>2}",
        style("=").red().bold(),
        style("=").green().bold(),
        style(NDAY).yellow().bold()
    );

    let start_time = Instant::now();
    let processed_input = TSln::get_input();
    let process_input_duration = start_time.elapsed();

    if process_input_duration.as_secs() >= 1 {
        println!(
            "{}: {:?}",
            style("Input").black().bold(),
            style(process_input_duration).yellow()
        )
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
