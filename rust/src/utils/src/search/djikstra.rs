use super::backtrace::backtrace_from_goal;
use crate::hash::{Map, MapBuilder};
use crate::heap::MinHeap;

pub fn djikstra<TState>(
    initial_state: &TState,
    generate_successors: impl Fn(&TState) -> Vec<(TState, u32)>,
    is_goal: impl Fn(&TState) -> bool,
) -> Option<Vec<TState>>
where
    TState: Clone + std::fmt::Debug + Eq + std::hash::Hash,
{
    let mut best_distance: Map<TState, u32> = Map::from_array([(initial_state.clone(), 0)]);
    let mut previous: Map<TState, TState> = Map::new();
    let mut goal: Option<TState> = None;

    let mut to_process: MinHeap<u32, TState> = MinHeap::new();
    to_process.push(0, initial_state.clone());

    while let Some((_, node)) = to_process.pop() {
        for (neighbor, cost) in generate_successors(&node) {
            if best_distance.contains_key(&neighbor) {
                continue;
            }

            let cost_to_neighbor_on_this_path = best_distance[&node] + cost;
            if cost_to_neighbor_on_this_path < *best_distance.get(&neighbor).unwrap_or(&u32::MAX) {
                best_distance.insert(neighbor.clone(), cost_to_neighbor_on_this_path);
                previous.insert(neighbor.clone(), node.clone());
                to_process.push(cost_to_neighbor_on_this_path, neighbor.clone());
            }
        }

        if is_goal(&node) {
            goal = Some(node.clone());
        }
    }

    goal.map(|g| backtrace_from_goal(&previous, &g))
}
