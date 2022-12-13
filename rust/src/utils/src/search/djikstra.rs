use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

pub fn djikstra<TState, FSuccessors, FGoal>(
    initial_state: &TState,
    generate_successors: FSuccessors,
    is_goal: FGoal,
) -> Option<Vec<TState>>
where
    FSuccessors: Fn(&TState) -> Vec<(TState, u32)>,
    FGoal: Fn(&TState) -> bool,
    TState: Clone + std::fmt::Debug + Eq + std::hash::Hash,
{
    djikstra_from_n_states(&[initial_state.clone()], generate_successors, is_goal)
}

pub fn djikstra_from_n_states<TState, FSuccessors, FGoal>(
    initial_states: &[TState],
    generate_successors: FSuccessors,
    is_goal: FGoal,
) -> Option<Vec<TState>>
where
    FSuccessors: Fn(&TState) -> Vec<(TState, u32)>,
    FGoal: Fn(&TState) -> bool,
    TState: Clone + std::fmt::Debug + Eq + std::hash::Hash,
{
    let mut visited: HashSet<TState> = HashSet::new();
    let mut to_process: PriorityQueue<TState, Reverse<u32>> = PriorityQueue::new();
    let mut best_distance: HashMap<TState, u32> =
        initial_states.iter().map(|s| (s.clone(), 0)).collect();
    let mut previous: HashMap<TState, TState> = HashMap::new();
    let mut goal: Option<TState> = None;

    for state in initial_states {
        to_process.push(state.clone(), Reverse(0));
    }

    while let Some((node, _)) = to_process.pop() {
        for (neighbor, cost) in generate_successors(&node) {
            if visited.contains(&neighbor) {
                continue;
            }

            let cost_to_neighbor_on_this_path = best_distance[&node] + cost;
            if cost_to_neighbor_on_this_path < *best_distance.get(&neighbor).unwrap_or(&u32::MAX) {
                *best_distance
                    .entry(neighbor.clone())
                    .or_insert(cost_to_neighbor_on_this_path) = cost_to_neighbor_on_this_path;
                *previous
                    .entry(neighbor.clone())
                    .or_insert_with(|| node.clone()) = node.clone();
            }

            to_process.push(neighbor, Reverse(cost_to_neighbor_on_this_path));
        }

        if is_goal(&node) {
            goal = Some(node.clone());
        }

        to_process.remove(&node);
        visited.insert(node);
    }

    goal.map(|g| {
        let mut path = vec![g];

        while let Some(n) = previous.get(path.last().unwrap()) {
            path.push(n.clone());
        }

        path.reverse();
        path
    })
}
