use prelude::*;

pub struct Day16;

#[derive(Debug)]
pub struct Sue {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

pub struct Criteria {
    children: u32,
    cats: u32,
    samoyeds: u32,
    pomeranians: u32,
    akitas: u32,
    vizslas: u32,
    goldfish: u32,
    trees: u32,
    cars: u32,
    perfumes: u32,
}

const CRITERIA: Criteria = Criteria {
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

fn get_criteria(line: &str, name: &str) -> Option<u32> {
    Regex::new(&format!(r#"{name}: (\d+)"#))
        .unwrap()
        .captures(line)
        .expect("Invalid regex")
        .map(inputs::Captures::new)
        .map(|mut c| c.next_u32())
}

impl AocSolution for Day16 {
    type Input = Vec<Sue>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (sue_str, criteria_str) = l.split_once(": ").unwrap();
                let [number] = inputs::n_positive_numbers::<1>(sue_str);

                Sue {
                    number,
                    children: get_criteria(criteria_str, "children"),
                    cats: get_criteria(criteria_str, "cats"),
                    samoyeds: get_criteria(criteria_str, "samoyeds"),
                    pomeranians: get_criteria(criteria_str, "pomeranians"),
                    akitas: get_criteria(criteria_str, "akitas"),
                    vizslas: get_criteria(criteria_str, "vizslas"),
                    goldfish: get_criteria(criteria_str, "goldfish"),
                    trees: get_criteria(criteria_str, "trees"),
                    cars: get_criteria(criteria_str, "cars"),
                    perfumes: get_criteria(criteria_str, "perfumes"),
                }
            })
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(103);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        input
            .iter()
            .find(|s| {
                s.children.map_or(true, |c| c == CRITERIA.children)
                    && s.cats.map_or(true, |c| c == CRITERIA.cats)
                    && s.samoyeds.map_or(true, |c| c == CRITERIA.samoyeds)
                    && s.pomeranians.map_or(true, |c| c == CRITERIA.pomeranians)
                    && s.akitas.map_or(true, |c| c == CRITERIA.akitas)
                    && s.vizslas.map_or(true, |c| c == CRITERIA.vizslas)
                    && s.goldfish.map_or(true, |c| c == CRITERIA.goldfish)
                    && s.trees.map_or(true, |c| c == CRITERIA.trees)
                    && s.cars.map_or(true, |c| c == CRITERIA.cars)
                    && s.perfumes.map_or(true, |c| c == CRITERIA.perfumes)
            })
            .unwrap()
            .number
    }

    const PART2_SOLUTION: Solution = solution(405);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        input
            .iter()
            .find(|s| {
                s.children.map_or(true, |c| c == CRITERIA.children)
                    && s.cats.map_or(true, |c| c > CRITERIA.cats)
                    && s.samoyeds.map_or(true, |c| c == CRITERIA.samoyeds)
                    && s.pomeranians.map_or(true, |c| c < CRITERIA.pomeranians)
                    && s.akitas.map_or(true, |c| c == CRITERIA.akitas)
                    && s.vizslas.map_or(true, |c| c == CRITERIA.vizslas)
                    && s.goldfish.map_or(true, |c| c < CRITERIA.goldfish)
                    && s.trees.map_or(true, |c| c > CRITERIA.trees)
                    && s.cars.map_or(true, |c| c == CRITERIA.cars)
                    && s.perfumes.map_or(true, |c| c == CRITERIA.perfumes)
            })
            .unwrap()
            .number
    }
}
