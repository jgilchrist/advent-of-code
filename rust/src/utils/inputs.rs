pub fn comma_separated_integers(s: &str) -> Vec<u32> {
    s.trim()
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}
