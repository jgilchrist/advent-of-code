use aoc::prelude::*;
use std::cmp::Ordering;
use utils::prelude::*;

pub struct Day05;

impl AocSolution for Day05 {
    type Input = (Vec<[u32; 2]>, Vec<Vec<u32>>);

    fn process_input(input: &str) -> Self::Input {
        let (rules, updates) = input.split_once("\n\n").unwrap();

        let rules = rules
            .lines()
            .map(|l| inputs::n_positive_numbers::<2>(l))
            .collect_vec();

        let updates = updates
            .lines()
            .map(|l| inputs::positive_numbers(l))
            .collect_vec();

        (rules, updates)
    }

    const PART1_SOLUTION: SolutionStatus = solution(5108);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let (rules, updates) = input;

        updates
            .iter()
            .map(|u| {
                let mut update = u.iter().copied().collect_vec();

                update.sort_by(|&a, &b| {
                    let matching_rule = rules
                        .iter()
                        .find(|r| (r[0] == a || r[1] == a) && (r[0] == b || r[1] == b));

                    matching_rule.map_or(Ordering::Equal, |r| {
                        if r[0] == a {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                });

                if update == *u {
                    update[update.len() / 2]
                } else {
                    0
                }
            })
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(7380);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let (rules, updates) = input;

        updates
            .iter()
            .map(|u| {
                let mut update = u.iter().copied().collect_vec();

                update.sort_by(|&a, &b| {
                    let matching_rule = rules
                        .iter()
                        .find(|r| (r[0] == a || r[1] == a) && (r[0] == b || r[1] == b));

                    matching_rule.map_or(Ordering::Equal, |r| {
                        if r[0] == a {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                });

                if update == *u {
                    0
                } else {
                    update[update.len() / 2]
                }
            })
            .sum::<u32>()
    }
}
