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

pub fn run_solution<TYear: AocYear, TSln: AocSolution>() -> Result<()> {
    if !TSln::SOLVED {
        return Ok(());
    }

    println!(
        "{}{} Day {:0>2}",
        style("=").red().bold(),
        style("=").green().bold(),
        style(TSln::DAY).yellow().bold()
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

pub fn run_year<TYear: AocYear>() -> Result<()> {
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
