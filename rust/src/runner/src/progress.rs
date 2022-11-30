#![allow(clippy::print_stdout)]

use aoc::{AocSolution, AocYear, Solution};
use console::style;

pub(crate) fn print_progress() {
    print_year_progress::<y2015::Y2015, 2015>();
    print_year_progress::<y2016::Y2016, 2016>();
    print_year_progress::<y2017::Y2017, 2017>();
    print_year_progress::<y2018::Y2018, 2018>();
    print_year_progress::<y2019::Y2019, 2019>();
    print_year_progress::<y2020::Y2020, 2020>();
    print_year_progress::<y2021::Y2021, 2021>();
    print_year_progress::<y2022::Y2022, 2022>();
}

fn print_year_progress<TYear: AocYear, const NYEAR: u32>() {
    print!(
        "{} {} ",
        style(NYEAR).blue().bold(),
        style("│").black().bold()
    );
    print_day_header::<TYear::D01, 1, NYEAR>();
    print_day_header::<TYear::D02, 2, NYEAR>();
    print_day_header::<TYear::D03, 3, NYEAR>();
    print_day_header::<TYear::D04, 4, NYEAR>();
    print_day_header::<TYear::D05, 5, NYEAR>();
    print_day_header::<TYear::D06, 6, NYEAR>();
    print_day_header::<TYear::D07, 7, NYEAR>();
    print_day_header::<TYear::D08, 8, NYEAR>();
    print_day_header::<TYear::D09, 9, NYEAR>();
    print_day_header::<TYear::D10, 10, NYEAR>();
    print_day_header::<TYear::D11, 11, NYEAR>();
    print_day_header::<TYear::D12, 12, NYEAR>();
    print_day_header::<TYear::D13, 13, NYEAR>();
    print_day_header::<TYear::D14, 14, NYEAR>();
    print_day_header::<TYear::D15, 15, NYEAR>();
    print_day_header::<TYear::D16, 16, NYEAR>();
    print_day_header::<TYear::D17, 17, NYEAR>();
    print_day_header::<TYear::D18, 18, NYEAR>();
    print_day_header::<TYear::D19, 19, NYEAR>();
    print_day_header::<TYear::D20, 20, NYEAR>();
    print_day_header::<TYear::D21, 21, NYEAR>();
    print_day_header::<TYear::D22, 22, NYEAR>();
    print_day_header::<TYear::D23, 23, NYEAR>();
    print_day_header::<TYear::D24, 24, NYEAR>();
    print_day_header::<TYear::D25, 25, NYEAR>();

    println!();
    print!("     {} ", style("│").black().bold());

    print_day_progress::<TYear::D01, 1, NYEAR>();
    print_day_progress::<TYear::D02, 2, NYEAR>();
    print_day_progress::<TYear::D03, 3, NYEAR>();
    print_day_progress::<TYear::D04, 4, NYEAR>();
    print_day_progress::<TYear::D05, 5, NYEAR>();
    print_day_progress::<TYear::D06, 6, NYEAR>();
    print_day_progress::<TYear::D07, 7, NYEAR>();
    print_day_progress::<TYear::D08, 8, NYEAR>();
    print_day_progress::<TYear::D09, 9, NYEAR>();
    print_day_progress::<TYear::D10, 10, NYEAR>();
    print_day_progress::<TYear::D11, 11, NYEAR>();
    print_day_progress::<TYear::D12, 12, NYEAR>();
    print_day_progress::<TYear::D13, 13, NYEAR>();
    print_day_progress::<TYear::D14, 14, NYEAR>();
    print_day_progress::<TYear::D15, 15, NYEAR>();
    print_day_progress::<TYear::D16, 16, NYEAR>();
    print_day_progress::<TYear::D17, 17, NYEAR>();
    print_day_progress::<TYear::D18, 18, NYEAR>();
    print_day_progress::<TYear::D19, 19, NYEAR>();
    print_day_progress::<TYear::D20, 20, NYEAR>();
    print_day_progress::<TYear::D21, 21, NYEAR>();
    print_day_progress::<TYear::D22, 22, NYEAR>();
    print_day_progress::<TYear::D23, 23, NYEAR>();
    print_day_progress::<TYear::D24, 24, NYEAR>();
    print_day_progress::<TYear::D25, 25, NYEAR>();

    println!();
    println!("     {} ", style("│").black().bold());
}

fn print_day_header<TSln: AocSolution, const NDAY: u32, const NYEAR: u32>() {
    print!(
        "{:0>2}  ",
        match (TSln::PART1_SOLUTION, TSln::PART2_SOLUTION) {
            (Solution::Solved(_), Solution::Solved(_)) => style(NDAY).yellow().bold(),
            _ => style(NDAY).black().bold(),
        }
    );
}

fn print_day_progress<TSln: AocSolution, const NDAY: u32, const NYEAR: u32>() {
    print!(
        "{}{}  ",
        match TSln::PART1_SOLUTION {
            Solution::Solved(_) | Solution::MerryChristmas => style("*").yellow().bold(),
            Solution::Wip | Solution::WipWithKnownAnswerFromPython(_) => style("*").blue().bold(),
            Solution::UnsolvedWithKnownAnswerFromPython(_) => style("*").black().bold(),
            Solution::Unsolved => style(" "),
        },
        match TSln::PART2_SOLUTION {
            Solution::Solved(_) | Solution::MerryChristmas => style("*").yellow().bold(),
            Solution::Wip | Solution::WipWithKnownAnswerFromPython(_) => style("*").blue().bold(),
            Solution::UnsolvedWithKnownAnswerFromPython(_) => style("*").black().bold(),
            Solution::Unsolved => style(" "),
        }
    );
}
