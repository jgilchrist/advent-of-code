use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

use crate::{utils::vecs::Vec2, AocSolution, aoc::Solution};

pub struct Day03;

static DIRECTION_MAP: Lazy<HashMap<char, Vec2>> = Lazy::new(|| {
    HashMap::from([
        ('^', Vec2::new(0, -1)),
        ('v', Vec2::new(0, 1)),
        ('<', Vec2::new(-1, 0)),
        ('>', Vec2::new(1, 0)),
    ])
});

impl AocSolution for Day03 {
    type Input = String;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d03.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.trim().to_string()
    }

    const PART1_SOLUTION: Solution<Self::Output> = Solution::Solution(2565);
    fn part1(input: &Self::Input) -> Self::Output {
        let mut pos = Vec2 { x: 0, y: 0 };
        let mut visited = HashSet::from([pos]);

        for char in input.chars() {
            let direction = DIRECTION_MAP[&char];
            pos += direction;
            visited.insert(pos);
        }

        visited.len()
    }

    const PART2_SOLUTION: Solution<Self::Output> = Solution::Solution(2639);
    fn part2(input: &Self::Input) -> Self::Output {
        let mut santa = Vec2 { x: 0, y: 0 };
        let mut robo_santa = Vec2 { x: 0, y: 0 };

        let mut visited = HashSet::from([santa, robo_santa]);

        let santas_instructions = input.chars().step_by(2).collect::<Vec<_>>();
        let robo_santas_instructions = input.chars().skip(1).step_by(2).collect::<Vec<_>>();

        for char in santas_instructions.iter() {
            let direction = DIRECTION_MAP[char];
            santa += direction;
            visited.insert(santa);
        }

        for char in robo_santas_instructions.iter() {
            let direction = DIRECTION_MAP[char];
            robo_santa += direction;
            visited.insert(robo_santa);
        }

        visited.len()
    }
}
