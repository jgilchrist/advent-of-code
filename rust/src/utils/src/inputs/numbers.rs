use fancy_regex::Regex;

pub fn positive_numbers(s: &str) -> Vec<u32> {
    let positive_numbers_regex: Regex = Regex::new(r"\d+").unwrap();

    positive_numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<u32>().unwrap())
        .collect()
}

pub fn n_positive_numbers<const N: usize>(s: &str) -> [u32; N] {
    let positive_numbers_regex = Regex::new(r"\d+").unwrap();

    positive_numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .expect("Incorrect number of numbers")
}

pub fn n_numbers<const N: usize>(s: &str) -> [i32; N] {
    let numbers_regex = Regex::new(r"-?\d+").unwrap();

    numbers_regex
        .find_iter(s)
        .map(|i| i.unwrap().as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .expect("Incorrect number of numbers")
}
