use fancy_regex::Regex;

pub fn comma_separated_integers(s: &str) -> Vec<u32> {
    s.trim()
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}

pub fn positive_numbers(s: &str) -> Vec<u32> {
    let positive_numbers_regex: Regex = Regex::new(r"\d+").unwrap();

    positive_numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}
