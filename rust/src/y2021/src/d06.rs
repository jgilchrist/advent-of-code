use prelude::*;

pub struct Day06;

fn number_of_fish_after_n_days(fish_start: &PopulationMap, days: u32) -> usize {
    let mut fish = fish_start.clone();

    for _ in 0..days {
        let mut new_fish: PopulationMap = PopulationMap::new();
        new_fish.store(0, fish.number(1));
        new_fish.store(1, fish.number(2));
        new_fish.store(2, fish.number(3));
        new_fish.store(3, fish.number(4));
        new_fish.store(4, fish.number(5));
        new_fish.store(5, fish.number(6));
        new_fish.store(6, fish.number(7) + fish.number(0));
        new_fish.store(7, fish.number(8));
        new_fish.store(8, fish.number(0));
        fish = new_fish;
    }

    fish.total()
}

#[derive(Clone, Debug)]
pub struct PopulationMap(HashMap<u8, usize>);

impl PopulationMap {
    fn new() -> Self {
        Self(HashMap::default())
    }

    fn store(&mut self, lifetime: u8, num: usize) {
        *self.0.entry(lifetime).or_default() = num;
    }

    fn increment(&mut self, lifetime: u8) {
        *self.0.entry(lifetime).or_default() += 1;
    }

    fn number(&self, lifetime: u8) -> usize {
        self.0.get(&lifetime).copied().unwrap_or(0)
    }

    fn total(&self) -> usize {
        self.0.values().sum()
    }
}

impl AocSolution for Day06 {
    type Input = PopulationMap;
    fn process_input(input: &str) -> Self::Input {
        let fish = inputs::comma_separated::<u8>(input);

        let mut fish_counts: PopulationMap = PopulationMap::new();
        for f in fish {
            fish_counts.increment(f);
        }

        fish_counts
    }

    const PART1_SOLUTION: Solution = solution(389726);
    fn part1(input: &Self::Input) -> impl ToSolution {
        number_of_fish_after_n_days(input, 80)
    }

    const PART2_SOLUTION: Solution = solution(1743335992042u64);
    fn part2(input: &Self::Input) -> impl ToSolution {
        number_of_fish_after_n_days(input, 256)
    }
}
