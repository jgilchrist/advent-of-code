use prelude::*;

pub struct Day15;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Debug, Clone)]
struct Cookie {
    amounts: HashMap<Ingredient, u32>,
}

impl Cookie {
    fn score_by(&self, getter: fn(&Ingredient) -> i32) -> i32 {
        self.amounts
            .iter()
            .map(|(ingredient, amount)| getter(ingredient) * *amount as i32)
            .sum::<i32>()
            .max(0)
    }

    fn score(&self) -> i32 {
        self.score_by(|i| i.capacity)
            * self.score_by(|i| i.durability)
            * self.score_by(|i| i.flavor)
            * self.score_by(|i| i.texture)
    }

    fn calories(&self) -> i32 {
        self.score_by(|i| i.calories)
    }
}

fn all_ingredient_combinations() -> impl Iterator<Item = [u32; 4]> {
    (0..=100u32).flat_map(move |x| {
        (0..=100 - x).flat_map(move |y| (0..=100 - x - y).map(move |z| [x, y, z, 100 - x - y - z]))
    })
}

fn all_cookies(ingredients: &Vec<Ingredient>) -> impl Iterator<Item = Cookie> + '_ {
    all_ingredient_combinations().map(|amounts| Cookie {
        amounts: ingredients
            .into_iter()
            .zip(amounts)
            .map(|(ingredient, amount)| (ingredient.clone(), amount))
            .collect(),
    })
}

impl AocSolution for Day15 {
    type Input = Vec<Ingredient>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(inputs::n_numbers)
            .map(
                |[capacity, durability, flavor, texture, calories]| Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            )
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(13882464);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        all_cookies(input)
            .max_by_key(|c| c.score())
            .unwrap()
            .score()
    }

    const PART2_SOLUTION: Solution = solution(11171160);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        all_cookies(input)
            .max_by_key(|c| if c.calories() == 500 { c.score() } else { 0 })
            .unwrap()
            .score()
    }
}
