use std::{cmp::Ordering};

use crate::{utils::{iters::count_items, vecs::Vec2}, AocSolution};

pub struct Day05;

fn direction_of_travel<T>(from: T, to: T) -> i32
where
    T: Ord,
{
    match from.cmp(&to) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

#[derive(Debug)]
pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Line {
        Line { start, end }
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn points(&self) -> Vec<Vec2> {
        let x_dir = direction_of_travel(self.start.x, self.end.x);
        let y_dir = direction_of_travel(self.start.y, self.end.y);
        let adjustment = Vec2 { x: x_dir, y: y_dir };

        let mut points = vec![self.start.clone()];
        let mut point = self.start.clone();

        while point != self.end {
            point = point + adjustment.clone();
            points.push(point.clone());
        }

        points
    }
}

fn parse_point(s: &str) -> Vec2 {
    let xy: Vec<_> = s.split(',').collect();

    let (x, y) = (xy[0].parse::<i32>().unwrap(), xy[1].parse::<i32>().unwrap());

    Vec2 { x, y }
}

fn parse_line(s: &str) -> Line {
    let points: Vec<_> = s.split(" -> ").collect();
    let (p1, p2) = (points[0], points[1]);

    let p1 = parse_point(p1);
    let p2 = parse_point(p2);

    Line::new(p1, p2)
}

impl AocSolution<5> for Day05 {
    type Input = Vec<Line>;
    type Output = usize;

    fn get_input() -> &'static str {
        include_str!("d05.in")
    }

    fn process_input(input: &str) -> Self::Input {
        input.lines().map(parse_line).collect()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(i: &Self::Input) -> Self::Output {
        let relevant_lines = i
            .iter()
            .filter(|line| line.is_horizontal() || line.is_vertical());

        let points = relevant_lines
            .flat_map(|line| line.points())
            .collect::<Vec<_>>();

        let counted_points = count_items(&points);

        counted_points.into_iter().filter(|&(_, v)| v >= 2).count()
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(i: &Self::Input) -> Self::Output {
        let points = i.iter().flat_map(|line| line.points()).collect::<Vec<_>>();

        let counted_points = count_items(&points);

        counted_points.into_iter().filter(|&(_, v)| v >= 2).count()
    }
}
