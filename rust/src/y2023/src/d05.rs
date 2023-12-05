use aoc::prelude::*;
use std::ops::Range;
use utils::prelude::*;

pub struct Day05;

#[derive(Debug)]
pub struct ProblemDefinition {
    seeds: Vec<u32>,
    maps: MapList,
}

#[derive(Debug)]
pub struct MapList {
    maps: Vec<Vec<Transform>>,
}

impl MapList {
    pub fn apply_maps(&self, mut n: u32) -> u32 {
        for map in &self.maps {
            n = self.apply_map(n, map);
        }

        n
    }

    fn apply_map(&self, n: u32, map: &Vec<Transform>) -> u32 {
        for t in map {
            let (n, did_transform) = t.transform(n);
            if did_transform {
                return n;
            }
        }

        n
    }
}

#[derive(Debug)]
pub struct Transform {
    range: Range<u32>,
    offset: i32,
}

impl Transform {
    pub fn transform(&self, n: u32) -> (u32, bool) {
        if self.range.contains(&n) {
            ((n as i32 + self.offset) as u32, true)
        } else {
            (n, false)
        }
    }
}

impl AocSolution for Day05 {
    type Input = ProblemDefinition;
    fn process_input(input: &str) -> Self::Input {
        let (seeds_block, map_block) = input.split_once("\n\n").unwrap();
        let map_blocks = map_block.split("\n\n");

        let seeds = inputs::positive_numbers(seeds_block);
        let maps = map_blocks
            .map(|block| {
                let (_, transforms) = block.split_once("\n").unwrap();

                transforms
                    .split("\n")
                    .filter(|ts| !ts.is_empty())
                    .map(|t| {
                        let [destination_range_start, range_start, length] =
                            inputs::n_positive_numbers(t);

                        Transform {
                            range: range_start..(range_start + length),
                            offset: destination_range_start as i32 - range_start as i32,
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        ProblemDefinition {
            seeds,
            maps: MapList { maps },
        }
    }

    const PART1_SOLUTION: SolutionStatus = solution(251346198);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .seeds
            .iter()
            .map(|seed| input.maps.apply_maps(*seed))
            .min()
            .unwrap()
    }

    const PART2_SOLUTION: SolutionStatus = SolutionStatus::Wip;
    fn part2(input: &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}
