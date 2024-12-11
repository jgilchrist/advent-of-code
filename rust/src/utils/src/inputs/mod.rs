mod grid;
mod numbers;
mod regex;
mod text;

pub use grid::grid_of;
pub use numbers::{n_numbers, n_positive_numbers, positive_numbers};
pub use regex::{match_in_string, match_per_line, transform_lines, Captures};
pub use text::{comma_separated, separated_by};

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|l| l.to_owned()).collect()
}
