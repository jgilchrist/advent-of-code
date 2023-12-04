use std::str::FromStr;

pub fn separated_by<T>(s: &str, separator: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.trim()
        .split(separator)
        .map(|i| i.parse::<T>().unwrap())
        .collect()
}

pub fn comma_separated<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    separated_by::<T>(s, ",")
}
