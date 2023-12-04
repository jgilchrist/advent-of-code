#![allow(clippy::print_stdout)]
#![allow(unused)]

use crate::{AocSolution, AocYear, SolutionStatus};
use console::style;

pub fn print_year_progress<TYear: AocYear>() {
    print!(
        "{} {} ",
        style(TYear::YEAR).blue().bold(),
        style("│").black().bold()
    );
    print_day_header::<TYear::D01, 1>();
    print_day_header::<TYear::D02, 2>();
    print_day_header::<TYear::D03, 3>();
    print_day_header::<TYear::D04, 4>();
    print_day_header::<TYear::D05, 5>();
    print_day_header::<TYear::D06, 6>();
    print_day_header::<TYear::D07, 7>();
    print_day_header::<TYear::D08, 8>();
    print_day_header::<TYear::D09, 9>();
    print_day_header::<TYear::D10, 10>();
    print_day_header::<TYear::D11, 11>();
    print_day_header::<TYear::D12, 12>();
    print_day_header::<TYear::D13, 13>();
    print_day_header::<TYear::D14, 14>();
    print_day_header::<TYear::D15, 15>();
    print_day_header::<TYear::D16, 16>();
    print_day_header::<TYear::D17, 17>();
    print_day_header::<TYear::D18, 18>();
    print_day_header::<TYear::D19, 19>();
    print_day_header::<TYear::D20, 20>();
    print_day_header::<TYear::D21, 21>();
    print_day_header::<TYear::D22, 22>();
    print_day_header::<TYear::D23, 23>();
    print_day_header::<TYear::D24, 24>();
    print_day_header::<TYear::D25, 25>();

    println!();
    print!("     {} ", style("│").black().bold());

    print_day_progress::<TYear::D01, 1>();
    print_day_progress::<TYear::D02, 2>();
    print_day_progress::<TYear::D03, 3>();
    print_day_progress::<TYear::D04, 4>();
    print_day_progress::<TYear::D05, 5>();
    print_day_progress::<TYear::D06, 6>();
    print_day_progress::<TYear::D07, 7>();
    print_day_progress::<TYear::D08, 8>();
    print_day_progress::<TYear::D09, 9>();
    print_day_progress::<TYear::D10, 10>();
    print_day_progress::<TYear::D11, 11>();
    print_day_progress::<TYear::D12, 12>();
    print_day_progress::<TYear::D13, 13>();
    print_day_progress::<TYear::D14, 14>();
    print_day_progress::<TYear::D15, 15>();
    print_day_progress::<TYear::D16, 16>();
    print_day_progress::<TYear::D17, 17>();
    print_day_progress::<TYear::D18, 18>();
    print_day_progress::<TYear::D19, 19>();
    print_day_progress::<TYear::D20, 20>();
    print_day_progress::<TYear::D21, 21>();
    print_day_progress::<TYear::D22, 22>();
    print_day_progress::<TYear::D23, 23>();
    print_day_progress::<TYear::D24, 24>();
    print_day_progress::<TYear::D25, 25>();

    println!();
    println!("     {} ", style("│").black().bold());
}

fn print_day_header<TSln: AocSolution, const NDAY: u32>() {
    print!(
        "{:0>2}  ",
        match (TSln::PART1_SOLUTION, TSln::PART2_SOLUTION) {
            (SolutionStatus::Solved(_), SolutionStatus::Solved(_)) => style(NDAY).yellow().bold(),
            _ => style(NDAY).black().bold(),
        }
    );
}

fn print_day_progress<TSln: AocSolution, const NDAY: u32>() {
    print!(
        "{}{}  ",
        solution_progress_str(&TSln::PART1_SOLUTION),
        solution_progress_str(&TSln::PART2_SOLUTION)
    );
}

fn solution_progress_str(status: &SolutionStatus) -> String {
    match status {
        SolutionStatus::Solved(_) => style("*").yellow().bold(),
        SolutionStatus::Wip => style("*").blue().bold(),
        SolutionStatus::SolvedInPython(_) => style("*").black().bold(),
        SolutionStatus::Unsolved => style(" "),
    }
    .to_string()
}
