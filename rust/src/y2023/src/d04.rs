use prelude::*;

pub struct Day04;

pub struct Game {
    winning_numbers: HashSet<u32>,
    our_numbers: HashSet<u32>,
}

impl Game {
    pub fn count_winning_cards(&self) -> u32 {
        self.winning_numbers.intersection(&self.our_numbers).count() as u32
    }
}

impl AocSolution for Day04 {
    type Input = Vec<Game>;
    fn process_input(input: &str) -> Self::Input {
        inputs::lines(input)
            .iter()
            .map(|line| {
                let (_, all_numbers) = line.split_once(": ").unwrap();
                let (winning_numbers_str, our_numbers_str) = all_numbers.split_once(" | ").unwrap();

                let (winning_numbers, our_numbers) = (
                    inputs::positive_numbers(winning_numbers_str),
                    inputs::positive_numbers(our_numbers_str),
                );

                Game {
                    winning_numbers: HashSet::from_iter(winning_numbers),
                    our_numbers: HashSet::from_iter(our_numbers),
                }
            })
            .collect()
    }

    const PART1_SOLUTION: SolutionStatus = solution(24175);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .map(|game| match game.count_winning_cards() {
                0 => 0,
                count => 2u32.pow(count - 1),
            })
            .sum::<u32>()
    }

    const PART2_SOLUTION: SolutionStatus = solution(18846301);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let winning_cards = input
            .iter()
            .map(|game| game.count_winning_cards())
            .collect_vec();

        let mut copies = HashMap::<usize, u32>::new();
        for card in 0..input.len() {
            copies.insert(card, 1);
        }

        // For every card...
        for card in 0..input.len() {
            let number_of_this_card = copies[&card];

            // For every copy of that card...
            for _ in 0..number_of_this_card {
                // Add copies of future cards...
                for next_card_idx in 1..=winning_cards[card] {
                    *copies.entry(card + next_card_idx as usize).or_default() += 1;
                }
            }
        }

        copies.values().sum::<u32>()
    }
}
