use aoc::{AocSolution, Solution};

pub struct Day02;

pub enum Instruction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn parse_instruction(l: &str) -> Instruction {
    let mut items = l.split_whitespace();
    let instruction = items.next().unwrap();

    let amount = items.next().unwrap().parse::<u32>().unwrap();

    match instruction {
        "up" => Instruction::Up(amount),
        "down" => Instruction::Down(amount),
        "forward" => Instruction::Forward(amount),
        _ => unreachable!(),
    }
}

impl AocSolution for Day02 {
    fn get_input() -> &'static str {
        include_str!("d02.in")
    }

    type Input = Vec<Instruction>;
    fn process_input(input: &str) -> Self::Input {
        input.lines().map(parse_instruction).collect()
    }

    type Part1Output = u32;
    const PART1_SOLUTION: Solution<Self::Part1Output> = Solution::Solved(1924923);
    fn part1(input: &Self::Input) -> Self::Part1Output {
        let mut depth = 0;
        let mut position = 0;

        for instruction in input.iter() {
            use Instruction::*;
            match instruction {
                Up(n) => depth -= n,
                Down(n) => depth += n,
                Forward(n) => position += n,
            }
        }

        depth * position
    }

    type Part2Output = u32;
    const PART2_SOLUTION: Solution<Self::Part2Output> = Solution::Solved(1982495697);
    fn part2(input: &Self::Input) -> Self::Part2Output {
        let mut aim = 0;
        let mut depth = 0;
        let mut position = 0;

        for instruction in input.iter() {
            use Instruction::*;
            match instruction {
                Up(n) => aim -= n,
                Down(n) => aim += n,
                Forward(n) => {
                    position += n;
                    depth += aim * n;
                }
            }
        }

        depth * position
    }
}
