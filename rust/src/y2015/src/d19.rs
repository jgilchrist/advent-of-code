use prelude::*;

pub struct Day19;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule(String, String);

fn apply_rule(s: &str, rule: &Rule) -> HashSet<String> {
    let Rule(before, after) = rule;
    let mut results: HashSet<String> = HashSet::new();

    for (idx, _) in s.match_indices(before) {
        let mut with_replacement = s.to_string();
        with_replacement.replace_range(idx..idx + before.len(), after);
        results.insert(with_replacement);
    }

    results
}

impl AocSolution for Day19 {
    type Input = (Vec<Rule>, String);
    fn process_input(input: &str) -> Self::Input {
        let (rules, initial_state) = input.split_once("\n\n").unwrap();

        let rules = rules.lines().map(|l| {
            let (before, after) = l.split_once(" => ").unwrap();
            Rule(before.to_owned(), after.to_owned())
        });

        (rules.collect(), initial_state.trim().to_owned())
    }

    const PART1_SOLUTION: Solution = solution(535);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        let (rules, initial_state) = input;
        let unique_states: HashSet<String> = rules
            .iter()
            .flat_map(|rule| apply_rule(initial_state, rule))
            .collect();
        unique_states.len()
    }

    const PART2_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART2_SOLUTION: Solution = Solution::Unsolved;
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
        // let (rules, goal_state) = input;
        //
        // let path =
        //     search::djikstra(
        //     &"e".to_string(),
        //     |state| rules.iter()
        //         .flat_map(|rule|
        //             apply_rule(state, rule)
        //                 .into_iter()
        //                 .map(|new_state| (new_state.clone(), 1))
        //         )
        //         .filter(|(s, _)| s.len() <= goal_state.len())
        //         .collect()
        //     ,
        //     |state| state == goal_state
        // ).unwrap();
        //
        // path.len() - 1
    }
}
