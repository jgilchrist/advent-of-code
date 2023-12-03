use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;

use crate::geometry::d2::grid::Grid;

#[derive(Debug)]
pub struct Captures<'a> {
    captures: fancy_regex::Captures<'a>,
    idx: usize,
}

impl<'a> Captures<'a> {
    pub fn new(captures: fancy_regex::Captures<'a>) -> Self {
        Self { captures, idx: 1 }
    }

    fn next_capture(&mut self) -> &str {
        let capture = self.captures.get(self.idx).expect("No capture").as_str();
        self.idx += 1;
        capture
    }

    fn try_next<T>(&mut self) -> Option<T>
    where
        T: FromStr,
    {
        let next_capture = self.next_capture();
        next_capture.parse::<T>().ok()
    }

    fn next<T>(&mut self) -> T
    where
        T: FromStr,
    {
        self.try_next().unwrap()
    }

    pub fn next_string(&mut self) -> String {
        self.next::<String>()
    }

    pub fn next_u8(&mut self) -> u8 {
        self.next::<u8>()
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next::<u32>()
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next::<i32>()
    }
}

type TransformRegexes<T> = Vec<(&'static str, TransformFn<T>)>;
type TransformFn<T> = Box<dyn Fn(Captures) -> T>;

pub fn regexes<T>(input: &str, regexes: TransformRegexes<T>) -> Vec<T> {
    let compiled_regexes = regexes
        .into_iter()
        .map(|(re, x)| ("^".to_owned() + re + "$", x))
        .map(|(re, x)| (Regex::new(&re).unwrap(), x))
        .collect_vec();

    let transformed_lines = input
        .lines()
        .map(|l| {
            let regexes: &[(Regex, TransformFn<T>)] = &compiled_regexes;
            for (regex, transform_fn) in regexes {
                if let Some(captures) = regex.captures(l).expect("Invalid regex") {
                    return transform_fn(Captures::new(captures));
                }
            }

            panic!("Did not match any regexes: \"{l}\"");
        })
        .collect();

    transformed_lines
}

pub fn regex_lines<'a>(input: &'a str, regex: &str) -> impl Iterator<Item = Captures<'a>> {
    let line_regex = "^".to_owned() + regex + "$";
    regex_matches(input, &line_regex)
}

pub fn regex_matches<'a>(input: &'a str, regex: &str) -> impl Iterator<Item = Captures<'a>> {
    let compiled_regex = Regex::new(regex).unwrap();

    input.lines().map(move |l| {
        Captures::new(
            compiled_regex
                .captures(l)
                .expect("Invalid regex")
                .unwrap_or_else(|| panic!("Did not match regex: {l}")),
        )
    })
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|l| l.to_owned()).collect()
}

pub fn separated_by<T>(s: &str, separator: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.trim()
        .split(separator)
        .map(|i| i.parse::<T>().unwrap())
        .collect()
}

pub fn comma_separated<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    separated_by::<T>(s, ",")
}

pub fn positive_numbers(s: &str) -> Vec<u32> {
    let positive_numbers_regex: Regex = Regex::new(r"\d+").unwrap();

    positive_numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<u32>().unwrap())
        .collect()
}

pub fn n_positive_numbers<const N: usize>(s: &str) -> [u32; N] {
    let positive_numbers_regex = Regex::new(r"\d+").unwrap();

    positive_numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .expect("Incorrect number of numbers")
}

pub fn n_numbers<const N: usize>(s: &str) -> [i32; N] {
    let numbers_regex = Regex::new(r"-?\d+").unwrap();

    numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .expect("Incorrect number of numbers")
}

pub fn grid_of<T>(s: &str) -> Grid<T>
where
    T: From<char>,
{
    let lines = s.lines().collect_vec();
    let x_size = lines[0].len();
    let y_size = lines.len();

    Grid::new(
        x_size,
        y_size,
        s.lines().join("").chars().map(|c| c.into()).collect(),
    )
}
