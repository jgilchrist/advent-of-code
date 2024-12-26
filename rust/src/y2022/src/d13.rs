use aoc::prelude::*;
use utils::prelude::*;

use std::cmp::Ordering;

pub struct Day13;

#[derive(PartialEq, Eq, Clone)]
pub enum Value {
    Number(i64),
    List(Vec<Value>),
}

impl Value {
    fn from(s: &str) -> Self {
        Self::from_json(&serde_json::from_str(s).unwrap())
    }

    fn from_json(json: &serde_json::Value) -> Self {
        use serde_json::Value::*;

        match json {
            Number(n) => Self::Number(n.as_i64().unwrap()),
            Array(elems) => Self::List(elems.iter().map(Self::from_json).collect()),
            _ => unreachable!(),
        }
    }

    fn list_of(n: i64) -> Self {
        Self::List(vec![Self::Number(n)])
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;

        match (&self, &other) {
            (Number(x), Number(y)) => x.cmp(y),
            (List(x), List(y)) => {
                for pair in x.iter().zip_longest(y) {
                    match pair {
                        EitherOrBoth::Both(l, r) => {
                            let comparison = l.cmp(r);
                            if comparison != Ordering::Equal {
                                return comparison;
                            }
                        }
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }

                Ordering::Equal
            }
            (Number(l), r @ List(_)) => Self::list_of(*l).cmp(r),
            (l @ List(_), Number(r)) => (*l).cmp(&Self::list_of(*r)),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AocSolution for Day13 {
    type Input = Vec<(Value, Value)>;
    fn process_input(input: &str) -> Self::Input {
        input
            .split("\n\n")
            .map(|pairs| {
                let (left, right) = pairs.split_once('\n').unwrap();
                (Value::from(left), Value::from(right))
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(5529);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .enumerate()
            .filter(|(_, (left, right))| left < right)
            .map(|(idx, _)| idx + 1)
            .sum::<usize>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(27690);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let divider_packets = [
            Value::List(vec![Value::list_of(2)]),
            Value::List(vec![Value::list_of(6)]),
        ];

        let mut all_patterns = input
            .iter()
            .flat_map(move |(left, right)| [left.clone(), right.clone()])
            .collect::<Vec<_>>();

        all_patterns.extend_from_slice(&divider_packets[..]);

        divider_packets
            .into_iter()
            .map(|div| all_patterns.iter().filter(|x| **x < div).count() + 1)
            .product::<usize>()
    }
}
