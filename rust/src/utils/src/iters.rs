use std::collections::HashMap;
use std::hash::Hash;

pub fn count_items<T>(items: &[T]) -> HashMap<&T, u32>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();

    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
}

pub fn transpose<T>(ts: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Copy,
{
    let len = ts.first().unwrap().len();

    (0..len)
        .map(|idx| ts.iter().map(|num| num[idx]).collect())
        .collect()
}
