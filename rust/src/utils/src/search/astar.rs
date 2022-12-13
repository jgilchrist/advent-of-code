use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap};

use super::backtrace::backtrace_from_goal;

pub fn astar<TState, FSuccessors, FHeuristic>(
    initial_state: &TState,
    generate_successors_fn: FSuccessors,
    heuristic_fn: FHeuristic,
) -> Option<Vec<TState>>
where
    FSuccessors: Fn(&TState) -> Vec<(TState, u32)>,
    FHeuristic: Fn(&TState) -> u32,
    TState: Clone + std::fmt::Debug + Eq + std::hash::Hash,
{
    let mut best_distance: HashMap<TState, u32> = HashMap::from([(initial_state.clone(), 0)]);
    let mut previous: HashMap<TState, TState> = HashMap::new();
    let mut goal: Option<TState> = None;

    let mut to_process: PriorityQueue<TState, Reverse<u32>> = PriorityQueue::new();
    to_process.push(initial_state.clone(), Reverse(0));

    while let Some((node, _)) = to_process.pop() {
        for (neighbor, cost) in generate_successors_fn(&node) {
            let cost_to_neighbor_on_this_path = best_distance[&node] + cost;
            if cost_to_neighbor_on_this_path < *best_distance.get(&neighbor).unwrap_or(&u32::MAX) {
                best_distance.insert(neighbor.clone(), cost_to_neighbor_on_this_path);
                previous.insert(neighbor.clone(), node.clone());
                to_process.push(
                    neighbor.clone(),
                    Reverse(cost_to_neighbor_on_this_path + heuristic_fn(&neighbor)),
                );
            }
        }

        if heuristic_fn(&node) == 0 {
            goal = Some(node.clone());
        }
    }

    goal.map(|g| backtrace_from_goal(&previous, &g))
}
