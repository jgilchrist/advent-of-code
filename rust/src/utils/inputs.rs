use fancy_regex::Regex;

pub struct Captures<'a>(fancy_regex::Captures<'a>);

impl Captures<'_> {
    pub fn get_string(&self, i: usize) -> String {
        self.0.get(i).expect("No capture").as_str().to_string()
    }

    pub fn get_u32(&self, i: usize) -> u32 {
        self.0
            .get(i)
            .expect("No capture")
            .as_str()
            .parse::<u32>()
            .expect("Unable to parse to u32")
    }

    pub fn get_i32(&self, i: usize) -> i32 {
        self.0
            .get(i)
            .expect("No capture")
            .as_str()
            .parse::<i32>()
            .expect("Unable to parse to i32")
    }
}

pub type TransformFn<T> = Box<dyn Fn(Captures) -> T>;

pub fn transform_lines_by_regex<T>(input: &str, regexes: Vec<(&str, TransformFn<T>)>) -> Vec<T> {
    let compiled_regexes = regexes
        .into_iter()
        .map(|(re, x)| (Regex::new(re).unwrap(), x))
        .collect::<Vec<_>>();

    let transformed_lines = input
        .lines()
        .map(|l| transform_line_by_regex(&compiled_regexes, l))
        .collect::<Vec<_>>();

    transformed_lines
}

fn transform_line_by_regex<T>(regexes: &[(Regex, TransformFn<T>)], line: &str) -> T {
    for (regex, transform_fn) in regexes.iter() {
        if let Ok(captures) = regex.captures(line) {
            return transform_fn(Captures(captures.unwrap()));
        }
    }

    panic!("Did not match any regexes: \"{}\"", line);
}

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
