use super::backtrace::backtrace_from_goal;
use crate::hash::{Map, MapBuilder};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub fn astar<TState>(
    initial_state: &TState,
    generate_successors_fn: impl Fn(&TState) -> Vec<(TState, u32)>,
    heuristic_fn: impl Fn(&TState) -> u32,
) -> Option<Vec<TState>>
where
    TState: Clone + std::fmt::Debug + Eq + std::hash::Hash,
{
    let mut best_distance: Map<TState, u32> = Map::from_array([(initial_state.clone(), 0)]);
    let mut previous: Map<TState, TState> = Map::new();
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
