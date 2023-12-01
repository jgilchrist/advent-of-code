use prelude::*;

pub struct Day08;

fn regex_replace(s: &str, regex: &str, replace_with: &str) -> String {
    Regex::new(regex)
        .unwrap()
        .replace_all(s, replace_with)
        .to_string()
}

fn count_chars_in_line(line: &str) -> usize {
    let line = &line[1..line.len() - 1];

    let line = regex_replace(line, r#"\\\\"#, ".");
    let line = regex_replace(&line, r#"\\\""#, ".");
    let line = regex_replace(&line, r#"\\x[0-9A-Fa-f]{2}"#, ".");

    line.len()
}

fn count_expanded_chars_in_line(line: &str) -> usize {
    let line = regex_replace(line, r#"""#, "..");
    let line = regex_replace(&line, r#"\\"#, "..");
    let line = r#"""#.to_owned() + &line + r#"""#;

    line.len()
}

impl AocSolution for Day08 {
    type Input = Vec<String>;
    fn process_input(input: &str) -> Self::Input {
        inputs::lines(input)
    }

    const PART1_SOLUTION: Solution = solution(1342);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let total_len: usize = input.iter().map(|l| l.len()).sum();
        let string_len: usize = input.iter().map(|l| count_chars_in_line(l)).sum();

        total_len - string_len
    }

    const PART2_SOLUTION: Solution = solution(2074);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let total_len: usize = input.iter().map(|l| l.len()).sum();
        let expanded_len: usize = input.iter().map(|l| count_expanded_chars_in_line(l)).sum();

        expanded_len - total_len
    }
}
