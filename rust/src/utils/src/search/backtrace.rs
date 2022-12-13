use std::collections::HashMap;

pub fn backtrace_from_goal<T>(previous: &HashMap<T, T>, start: &T) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut path = vec![start.clone()];

    while let Some(n) = previous.get(path.last().unwrap()) {
        path.push(n.clone());
    }

    path.reverse();
    path
}
