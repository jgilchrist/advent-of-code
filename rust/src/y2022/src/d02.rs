use prelude::*;

pub struct Day02;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<char> for Play {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

enum PlayResult {
    Won,
    Drew,
    Lost,
}

impl PlayResult {
    fn score(&self) -> u32 {
        match self {
            Self::Won => 6,
            Self::Drew => 3,
            Self::Lost => 0,
        }
    }
}

impl From<char> for PlayResult {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Lost,
            'Y' => Self::Drew,
            'Z' => Self::Won,
            _ => unreachable!(),
        }
    }
}

fn play_game(rounds: impl Iterator<Item = (Play, Play)>) -> u32 {
    rounds
        .map(|(them, me)| play(them, me).score() + me.score())
        .sum::<u32>()
}

fn play(them: Play, me: Play) -> PlayResult {
    use Play::*;
    use PlayResult::*;
    match (them, me) {
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Drew,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Won,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Lost,
    }
}

fn desired_play(them: Play, my_desired_result: PlayResult) -> Play {
    use Play::*;
    use PlayResult::*;
    match (them, my_desired_result) {
        (Rock, Drew) | (Scissors, Won) | (Paper, Lost) => Rock,
        (Paper, Drew) | (Rock, Won) | (Scissors, Lost) => Paper,
        (Scissors, Drew) | (Paper, Won) | (Rock, Lost) => Scissors,
    }
}

impl AocSolution for Day02 {
    type Input = Vec<(char, char)>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| l.chars().collect_vec())
            .map(|l| (*l.first().unwrap(), *l.get(2).unwrap()))
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(8392);
    fn part1(input: &Self::Input) -> impl ToSolution {
        play_game(
            input
                .iter()
                .map(|&(them, me)| (Into::<Play>::into(them), Into::<Play>::into(me))),
        )
    }

    const PART2_SOLUTION: Solution = solution(10116);
    fn part2(input: &Self::Input) -> impl ToSolution {
        play_game(
            input
                .iter()
                .map(|&(them, my_desired_result)| {
                    (
                        Into::<Play>::into(them),
                        Into::<PlayResult>::into(my_desired_result),
                    )
                })
                .map(|(them, my_desired_result)| (them, desired_play(them, my_desired_result))),
        )
    }
}
