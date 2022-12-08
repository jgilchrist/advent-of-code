mod take_until;

pub use take_until::TakeUntilExt;

pub fn transpose<T>(ts: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Copy,
{
    let len = ts.first().unwrap().len();

    (0..len)
        .map(|idx| ts.iter().map(|num| num[idx]).collect())
        .collect()
}
