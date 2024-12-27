use aoc::prelude::*;
use std::iter::Peekable;
use std::str::Chars;
use utils::prelude::*;

pub struct Day12;

#[derive(PartialEq, Eq)]
pub enum Json {
    Object(Map<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(i32),
}

fn count_numbers(json: &Json) -> i32 {
    use Json::*;

    match json {
        String(_) => 0,
        Number(n) => *n,
        Array(elems) => elems.iter().map(count_numbers).sum(),
        Object(obj) => obj.values().map(|v| count_numbers(v)).sum(),
    }
}

fn count_numbers_ignoring_red(json: &Json) -> i32 {
    use Json::*;

    match json {
        String(_) => 0,
        Number(n) => *n,
        Array(elems) => elems.iter().map(count_numbers_ignoring_red).sum(),
        Object(obj) => {
            let contains_red = obj
                .iter()
                .any(|(_, value)| *value == String("red".to_string()));

            if contains_red {
                0
            } else {
                obj.iter()
                    .map(|(_, prop_value)| count_numbers_ignoring_red(prop_value))
                    .sum()
            }
        }
    }
}

fn parse_number(s: &mut Peekable<Chars>) -> Json {
    Json::Number(
        s.peeking_take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap(),
    )
}

fn parse_string(s: &mut Peekable<Chars>) -> Json {
    // Consume the initial "
    assert_eq!(s.next(), Some('"'));

    let result = s.peeking_take_while(|c| *c != '"').collect::<String>();

    // Consume the final "
    assert_eq!(s.next(), Some('"'));

    Json::String(result)
}

fn parse_array(s: &mut Peekable<Chars>) -> Json {
    // Consume the initial [
    assert_eq!(s.next(), Some('['));

    let mut values = Vec::new();
    while s.peek().copied() != Some(']') {
        values.push(parse_json(s));

        // If we've got more elements, consume the connecting ,
        if s.peek().copied() == Some(',') {
            s.next();
        }
    }

    // Consume the final ]
    s.next();
    Json::Array(values)
}

fn parse_object(s: &mut Peekable<Chars>) -> Json {
    // Consume the initial {
    assert_eq!(s.next(), Some('{'));

    let mut map = Map::new();

    while s.peek().copied() != Some('}') {
        let key = parse_string(s);
        assert_eq!(s.next(), Some(':'));
        let value = parse_json(s);

        let Json::String(key) = key else {
            unreachable!()
        };

        map.insert(key, value);

        // If we've got more elements, consume the connecting ,
        if s.peek().copied() == Some(',') {
            s.next();
        }
    }

    // Consume the final }
    s.next();

    Json::Object(map)
}

fn parse_json(s: &mut Peekable<Chars>) -> Json {
    match s.peek().unwrap() {
        c if c.is_ascii_digit() => parse_number(s),
        '-' => parse_number(s),
        '"' => parse_string(s),
        '[' => parse_array(s),
        '{' => parse_object(s),
        _ => unreachable!(),
    }
}

impl AocSolution for Day12 {
    type Input = Json;
    fn process_input(input: &str) -> Self::Input {
        let mut chars = input.chars().peekable();
        parse_json(&mut chars)
    }

    const PART1_SOLUTION: SolutionStatus = solution(119433);
    fn part1(input: &Self::Input) -> impl ToSolution {
        count_numbers(input)
    }

    const PART2_SOLUTION: SolutionStatus = solution(68466);
    fn part2(input: &Self::Input) -> impl ToSolution {
        count_numbers_ignoring_red(input)
    }
}
