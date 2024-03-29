use aoc::prelude::*;
use utils::prelude::*;

pub struct Day14;

#[derive(Debug, Clone, Copy)]
pub struct Reindeer {
    fly_speed: u32,
    fly_period: u32,
    rest_period: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReindeerAction {
    Flying,
    Resting,
}

impl ReindeerAction {
    fn flip(&self) -> Self {
        use ReindeerAction::*;
        match *self {
            Flying => Resting,
            Resting => Flying,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReindeerState {
    action: ReindeerAction,
    time_until_transition: u32,
    distance: u32,
    score: u32,
}

impl AocSolution for Day14 {
    type Input = Vec<(Reindeer, ReindeerState)>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(inputs::n_positive_numbers)
            .map(|[fly_speed, fly_period, rest_period]| Reindeer {
                fly_speed,
                fly_period,
                rest_period,
            })
            .map(|r| {
                (
                    r,
                    ReindeerState {
                        action: ReindeerAction::Flying,
                        time_until_transition: r.fly_period,
                        distance: 0,
                        score: 0,
                    },
                )
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(2696);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut states = (*input).clone();

        for _ in 0..2503 {
            for r in &mut states {
                let (reindeer, state) = r;
                state.time_until_transition -= 1;

                if state.action == ReindeerAction::Flying {
                    state.distance += reindeer.fly_speed;
                }

                if state.time_until_transition == 0 {
                    state.action = state.action.flip();

                    state.time_until_transition = match state.action {
                        ReindeerAction::Flying => reindeer.fly_period,
                        ReindeerAction::Resting => reindeer.rest_period,
                    }
                }
            }
        }

        states.iter().map(|r| r.1.distance).max().unwrap()
    }

    const PART2_SOLUTION: SolutionStatus = solution(1084);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut input = (*input).clone();

        for _ in 0..2503 {
            let mut max_distance_this_go = 0;

            for r in &mut input {
                let (reindeer, state) = r;
                state.time_until_transition -= 1;

                if state.action == ReindeerAction::Flying {
                    state.distance += reindeer.fly_speed;
                }

                if state.distance > max_distance_this_go {
                    max_distance_this_go = state.distance;
                }

                if state.time_until_transition == 0 {
                    state.action = state.action.flip();

                    state.time_until_transition = match state.action {
                        ReindeerAction::Flying => reindeer.fly_period,
                        ReindeerAction::Resting => reindeer.rest_period,
                    }
                }
            }

            for r in &mut input {
                let (_, state) = r;
                if state.distance == max_distance_this_go {
                    state.score += 1;
                }
            }
        }

        input.iter().map(|r| r.1.score).max().unwrap()
    }
}
