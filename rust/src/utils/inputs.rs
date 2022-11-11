use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
pub struct Captures<'a>(pub fancy_regex::Captures<'a>);

impl Captures<'_> {
    pub fn get_string(&self, i: usize) -> String {
        self.0.get(i).expect("No capture").as_str().to_owned()
    }

    pub fn get_u8(&self, i: usize) -> u8 {
        self.0
            .get(i)
            .expect("No capture")
            .as_str()
            .parse::<u8>()
            .expect("Unable to parse to u8")
    }

    pub fn get_u32(&self, i: usize) -> u32 {
        self.0
            .get(i)
            .expect("No capture")
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse to u32")
    }

    pub fn get_i32(&self, i: usize) -> i32 {
        self.0
            .get(i)
            .expect("No capture")
            .as_str()
            .parse::<i32>()
            .expect("Unable to parse to i32")
    }
}

pub type TransformRegexes<T> = Vec<(&'static str, TransformFn<T>)>;
pub type TransformFn<T> = Box<dyn Fn(Captures) -> T>;

pub fn transform_lines_by_regex<T>(input: &str, regexes: TransformRegexes<T>) -> Vec<T> {
    let compiled_regexes = regexes
        .into_iter()
        .map(|(re, x)| ("^".to_owned() + re + "$", x))
        .map(|(re, x)| (Regex::new(&re).unwrap(), x))
        .collect_vec();

    let transformed_lines = input
        .lines()
        .map(|l| transform_line_by_regex(&compiled_regexes, l))
        .collect_vec();

    transformed_lines
}

fn transform_line_by_regex<T>(regexes: &[(Regex, TransformFn<T>)], line: &str) -> T {
    for (regex, transform_fn) in regexes.iter() {
        if let Some(captures) = regex.captures(line).expect("Invalid regex") {
            return transform_fn(Captures(captures));
        }
    }

    panic!("Did not match any regexes: \"{line}\"");
}

pub fn lines(s: &str) -> Vec<String> {
    s.lines().map(|l| l.to_owned()).collect()
}

pub fn comma_separated<T>(s: &str) -> Vec<T> where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {
    s.trim()
        .split(',')
        .map(|i| i.parse::<T>().unwrap())
        .collect()
}

pub fn positive_numbers(s: &str) -> Vec<u32> {
    let positive_numbers_regex: Regex = Regex::new(r"\d+").unwrap();

    positive_numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<u32>().unwrap())
        .collect_vec()
}
