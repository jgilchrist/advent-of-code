use std::collections::{HashMap, HashSet};

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
    let mut to_process: HashSet<TState> = initial_states.iter().cloned().collect();
    let mut best_distance: HashMap<TState, u32> =
        initial_states.iter().map(|s| (s.clone(), 0)).collect();
    let mut previous: HashMap<TState, TState> = HashMap::new();
    let mut goal: Option<TState> = None;

    let get_next_node_to_process =
        |nodes: &HashSet<TState>, distances: &HashMap<TState, u32>| -> Option<TState> {
            let n = nodes
                .iter()
                .map(|state| (state, distances.get(state).unwrap_or(&u32::MAX)))
                .min_by_key(|&(_, cost)| cost)
                .map(|(state, _)| state.clone());

            n
        };

    while let Some(node) = get_next_node_to_process(&to_process, &best_distance) {
        for (neighbor, cost) in generate_successors(&node) {
            let cost_to_neighbor_on_this_path = best_distance[&node] + cost;
            if cost_to_neighbor_on_this_path < *best_distance.get(&neighbor).unwrap_or(&u32::MAX) {
                *best_distance
                    .entry(neighbor.clone())
                    .or_insert(cost_to_neighbor_on_this_path) = cost_to_neighbor_on_this_path;
                *previous
                    .entry(neighbor.clone())
                    .or_insert_with(|| node.clone()) = node.clone();
            }

            if !visited.contains(&neighbor) {
                to_process.insert(neighbor);
            }
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
