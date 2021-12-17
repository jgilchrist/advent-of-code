use std::ops::Add;

use crate::{utils::count_items, AocSolution};

pub struct Day05;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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
        let x_dir = match self.start.x.cmp(&self.end.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let y_dir = match self.start.y.cmp(&self.end.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

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

fn parse_line(s: &str) -> Line {
    let points: Vec<_> = s.split(" -> ").collect();
    let (p1, p2) = (points[0], points[1]);

    let p1_xy: Vec<_> = p1.split(',').collect();
    let p2_xy: Vec<_> = p2.split(',').collect();

    let (p1_x, p1_y) = (
        p1_xy[0].parse::<i32>().unwrap(),
        p1_xy[1].parse::<i32>().unwrap(),
    );
    let (p2_x, p2_y) = (
        p2_xy[0].parse::<i32>().unwrap(),
        p2_xy[1].parse::<i32>().unwrap(),
    );

    Line::new(Vec2 { x: p1_x, y: p1_y }, Vec2 { x: p2_x, y: p2_y })
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
