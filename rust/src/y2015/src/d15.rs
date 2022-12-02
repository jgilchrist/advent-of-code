use rand::Rng;

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

// 100, 4 -> [34, 3, 21, 42]
fn randomly_partition_into_buckets(n: u32, buckets: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut randoms = (0..buckets - 1)
        .map(|_| rng.gen_range(0..n))
        .sorted_unstable()
        .collect_vec();
    randoms.insert(0, 0);
    randoms.push(n);
    randoms.array_windows().map(|[x, y]| y - x).collect_vec()
}

// Because any negative amounts become 0-terms in our product, there are areas of the state
// space which we can't use. We need to try a few different distributions of ingredients to
// find one that is usable.
fn get_initial_cookie(ingredients: &Vec<Ingredient>, heuristic: fn(&Cookie) -> i32) -> Cookie {
    let number_of_ingredients = ingredients.len();
    let mut loops = 0;

    loop {
        let ingredient_amounts = randomly_partition_into_buckets(100, number_of_ingredients);

        let cookie = Cookie {
            amounts: ingredients
                .into_iter()
                .zip(ingredient_amounts)
                .map(|(ingredient, amount)| (ingredient.clone(), amount))
                .collect(),
        };

        if heuristic(&cookie) > 0 {
            return cookie;
        }

        loops += 1;
        if loops > 100 {
            panic!();
        }
    }
}

fn find_best_cookie(ingredients: &Vec<Ingredient>, heuristic: fn(&Cookie) -> i32) -> Cookie {
    let mut possible_delta_components = vec![1, -1];
    possible_delta_components.extend(std::iter::repeat(0).take(ingredients.len() - 2));
    let possible_deltas = possible_delta_components
        .iter()
        .permutations(possible_delta_components.len())
        .collect_vec();

    let mut best_cookie = get_initial_cookie(ingredients, heuristic);
    let mut current_best_heuristic = heuristic(&best_cookie);

    loop {
        let new_cookies = possible_deltas
            .iter()
            .filter_map(|deltas| {
                let amounts: HashMap<Ingredient, i32> = best_cookie
                    .amounts
                    .iter()
                    .zip(deltas)
                    .map(|((ingredient, current_amount), delta)| {
                        (ingredient.clone(), (*current_amount as i32 + *delta))
                    })
                    .collect();

                if amounts.values().any(|a| *a < 0) {
                    return None;
                }

                Some(Cookie {
                    amounts: amounts
                        .into_iter()
                        .map(|(ingredient, current_amount)| (ingredient, current_amount as u32))
                        .collect(),
                })
            })
            .collect_vec();

        let best_cookie_this_round = new_cookies
            .iter()
            .max_by_key(|c| heuristic(c) - current_best_heuristic)
            .unwrap();

        let this_rounds_best_heuristic = heuristic(best_cookie_this_round);

        // None of our changes to ingredients improved the cookie, so we've found our optimal
        // (or at least a local maxima).
        if this_rounds_best_heuristic <= current_best_heuristic {
            break;
        }

        best_cookie = best_cookie_this_round.clone();
        current_best_heuristic = this_rounds_best_heuristic;
    }

    best_cookie
}

impl AocSolution for Day15 {
    type Input = Vec<Ingredient>;
    fn process_input(input: &str) -> Self::Input {
        inputs::regex_lines(input, r#"\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#)
            .map(|mut l| Ingredient {
                capacity: l.next_i32(),
                durability: l.next_i32(),
                flavor: l.next_i32(),
                texture: l.next_i32(),
                calories: l.next_i32(),
            }).collect()
    }

    const PART1_SOLUTION: Solution = solution(13882464);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        find_best_cookie(input, |c| c.score()).score()
    }

    const PART2_STATUS: SolutionStatus = SolutionStatus::Wip;
    const PART2_SOLUTION: Solution = Solution::Unsolved;
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let best_cookie = find_best_cookie(input, |c| dbg!(if dbg!(c.calories()) <= 500 {
            c.score()
        } else {
            0
        }));

        dbg!(best_cookie.calories());
        best_cookie.score()
    }
}
