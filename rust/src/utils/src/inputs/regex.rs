use fancy_regex::Regex;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
pub struct Captures<'a> {
    captures: fancy_regex::Captures<'a>,
    idx: usize,
}

impl<'a> Captures<'a> {
    pub fn new(captures: fancy_regex::Captures<'a>) -> Self {
        Self { captures, idx: 1 }
    }

    pub fn full(&mut self) -> &str {
        self.captures.get(0).expect("No capture").as_str()
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
        self.try_next().expect("No match")
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

pub fn transform_lines<T>(input: &str, regexes: TransformRegexes<T>) -> Vec<T> {
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

pub fn match_per_line<'a>(input: &'a str, regex: &str) -> impl Iterator<Item = Captures<'a>> {
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

pub fn matches_in_str<'a>(input: &'a str, regex: &str) -> impl Iterator<Item = Captures<'a>> {
    let compiled_regex = Regex::new(regex).unwrap();

    compiled_regex
        .captures_iter(input)
        .collect_vec()
        .into_iter()
        .map(move |l| Captures::new(l.expect("Invalid regex")))
}

pub fn match_in_string<'a>(input: &'a str, regex: &str) -> Option<Captures<'a>> {
    let compiled_regex = Regex::new(regex).unwrap();

    let captures = compiled_regex.captures(input).expect("Invalid regex");

    captures.map(|c| Captures::new(c))
}
