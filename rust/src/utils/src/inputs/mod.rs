mod grid;
mod numbers;
mod regex;
mod text;

pub use grid::grid_of;
pub use numbers::{n_numbers, n_positive_numbers, positive_numbers};
pub use regex::{line_regex_matches, regex_lines, regex_matches, regexes, Captures};
pub use text::{comma_separated, separated_by};

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|l| l.to_owned()).collect()
}
