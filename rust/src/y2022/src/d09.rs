use prelude::*;
use utils::geometry::d2::{coordinates::CardinalDirection, vecs::Vec2};

pub struct Day09;

impl AocSolution for Day09 {
    type Input = Vec<CardinalDirection>;
    fn process_input(input: &str) -> Self::Input {
        input.lines()
            .flat_map(|l| {
                let (dir, amount) = l.split_once(' ').unwrap();

                let dir = match dir {
                    "U" => CardinalDirection::North,
                    "D" => CardinalDirection::South,
                    "L" => CardinalDirection::West,
                    "R" => CardinalDirection::East,
                     _ => unreachable!(),
                };

                let amount = amount.parse::<usize>().unwrap();

                std::iter::repeat(dir).take(amount)
            })
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(6057);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        let mut head = Vec2::new(0, 0);
        let mut tail = Vec2::new(0, 0);

        let mut visited_coords: HashSet<Vec2> = HashSet::new();

        for &direction in input.into_iter() {
            head = head.move_in_direction(direction);
            tail = move_tail(&head, &tail);
            visited_coords.insert(tail);
        }

        visited_coords.len()
    }

    const PART2_SOLUTION: Solution = solution(2514);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        let mut sections = vec![Vec2::new(0, 0); 10];
        let mut visited_coords: HashSet<Vec2> = HashSet::new();

        for &direction in input.into_iter() {
            // Move the head
            sections[0] = sections[0].move_in_direction(direction);

            for idx in 0..9 {
                sections[idx + 1] = move_tail(&sections[idx], &sections[idx + 1])
            }

            visited_coords.insert(sections[9]);
        }

        visited_coords.len()
    }
}

fn move_tail(head: &Vec2, tail: &Vec2) -> Vec2 {
    dbg!(head, tail);

    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        tail.clone()
    } else {
        // We'll be moving horizontally
        if head.y == tail.y && head.x != tail.x {
            let x_direction = (head.x - tail.x).signum();
            dbg!("tail moved x", x_direction);
            *tail + Vec2::new(x_direction, 0)
        } else if head.x == tail.x && head.y != tail.y {
            let y_direction = (head.y - tail.y).signum();
            dbg!("tail moved y", y_direction);
            *tail + Vec2::new(0, y_direction)
        } else {
            let x_direction = (head.x - tail.x).signum();
            let y_direction = (head.y - tail.y).signum();
            dbg!("tail moved diagonal", Vec2::new(x_direction, y_direction));
            *tail + Vec2::new(x_direction, y_direction)
        }
    }
}
