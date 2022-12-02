use std::str::FromStr;

use fancy_regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
pub struct Captures<'a> {
    captures: fancy_regex::Captures<'a>,
    idx: usize,
}

impl<'a> Captures<'a> {
    fn new(captures: fancy_regex::Captures<'a>) -> Self {
        Self {
            captures,
            idx: 1,
        }
    }

    fn next_capture(&mut self) -> &str {
        let capture = self.captures.get(self.idx).expect("No capture").as_str();
        self.idx += 1;
        capture
    }

    pub fn next_string(&mut self) -> String {
        self.next_capture().to_owned()
    }

    pub fn next_u8(&mut self) -> u8 {
        self.next_capture()
            .parse::<u8>()
            .expect("Unable to parse to u8")
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_capture()
            .parse::<u32>()
            .expect("Unable to parse to u32")
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next_capture()
            .parse::<i32>()
            .expect("Unable to parse to i32")
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
            for (regex, transform_fn) in regexes.iter() {
                if let Some(captures) = regex.captures(l).expect("Invalid regex") {
                    return transform_fn(Captures::new(captures));
                }
            }

            panic!("Did not match any regexes: \"{l}\"");
        })
        .collect_vec();

    transformed_lines
}

pub fn regex_lines<'a>(input: &'a str, regex: &'static str) -> impl Iterator<Item = Captures<'a>> {
    let compiled_regex = Regex::new(&("^".to_owned() + regex + "$")).unwrap();

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
        .collect_vec()
}
