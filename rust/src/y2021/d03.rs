use crate::{
    utils::{count_items, transpose},
    AocSolution,
};

pub struct Day03;

fn to_decimal(chars: &[char]) -> u32 {
    let s = String::from_iter(chars);
    u32::from_str_radix(&s, 2).unwrap()
}

fn most_common_in(items: &[char]) -> char {
    let counts = count_items(items);

    let (most_common, most_common_count) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    if counts
        .iter()
        .filter(|(_, count)| *most_common_count == **count)
        .count()
        > 1
    {
        return '1';
    }

    **most_common
}

fn least_common_in(items: &[char]) -> char {
    let counts = count_items(items);

    let (least_common, least_common_count) = counts.iter().min_by_key(|&(_, count)| count).unwrap();
    if counts
        .iter()
        .filter(|(_, count)| *least_common_count == **count)
        .count()
        > 1
    {
        return '0';
    }

    **least_common
}

impl AocSolution<3> for Day03 {
    type Input = Vec<Vec<char>>;
    type Output = u32;

    fn get_input() -> &'static str {
        include_str!("d03.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| l.chars().collect())
            .collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = Some(2261546);
    fn part1(i: &Self::Input) -> Self::Output {
        let cols: Vec<Vec<char>> = transpose(i);

        let gamma_rate_chars: Vec<char> = cols.iter().map(|c| most_common_in(c)).collect();
        let epsilon_rate_chars: Vec<char> = cols.iter().map(|c| least_common_in(c)).collect();

        let gamma_rate = to_decimal(&gamma_rate_chars);
        let epsilon_rate = to_decimal(&epsilon_rate_chars);

        gamma_rate * epsilon_rate
    }

    const PART2_SOLUTION: Option<Self::Output> = Some(6775520);
    fn part2(i: &Self::Input) -> Self::Output {
        let mut oxygen_candidates = i.clone();

        let mut current_oxygen_bit_idx = 0;
        while oxygen_candidates.len() != 1 {
            let oxygen_candidate_cols = transpose(&oxygen_candidates);
            let most_common = most_common_in(&oxygen_candidate_cols[current_oxygen_bit_idx]);
            oxygen_candidates = oxygen_candidates
                .into_iter()
                .filter(|c| c[current_oxygen_bit_idx] == most_common)
                .collect();
            current_oxygen_bit_idx += 1;
        }

        let mut co2_candidates = i.clone();

        let mut current_co2_bit_idx = 0;
        while co2_candidates.len() != 1 {
            let co2_candidate_cols = transpose(&co2_candidates);
            let least_common = least_common_in(&co2_candidate_cols[current_co2_bit_idx]);
            co2_candidates = co2_candidates
                .into_iter()
                .filter(|c| c[current_co2_bit_idx] == least_common)
                .collect();
            current_co2_bit_idx += 1;
        }

        let oxygen_candidate = to_decimal(oxygen_candidates.first().unwrap());
        let co2_candidate = to_decimal(co2_candidates.first().unwrap());

        oxygen_candidate * co2_candidate
    }
}
