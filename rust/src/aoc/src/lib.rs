#![feature(return_position_impl_trait_in_trait)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::approx_constant)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]

pub enum SolutionStatus {
    Solved,
    SolvedInPython,
    Wip,
    Unsolved,
}

pub enum Solution {
    Number(i64),
    String(&'static str),
    MerryChristmas,
    Unsolved,
}

impl const From<&'static str> for Solution {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

impl From<String> for Solution {
    fn from(s: String) -> Self {
        // Used to be able to compare runtime-generated String results with
        // static strings embedded in the binary. There's likely a better
        // way to do this, but I haven't found it.
        // This is fine since the process lives only long enough to run the solution.
        let leaked_string = Box::leak(s.into_boxed_str());
        Self::String(leaked_string)
    }
}

#[allow(clippy::cast_lossless)]
impl const From<u16> for Solution {
    fn from(n: u16) -> Self {
        Self::Number(n as i64)
    }
}

#[allow(clippy::cast_lossless)]
impl const From<u32> for Solution {
    fn from(n: u32) -> Self {
        Self::Number(n as i64)
    }
}

#[allow(clippy::cast_lossless)]
impl const From<u64> for Solution {
    fn from(n: u64) -> Self {
        Self::Number(n as i64)
    }
}

#[allow(clippy::cast_lossless)]
impl const From<usize> for Solution {
    fn from(n: usize) -> Self {
        Self::Number(n as i64)
    }
}

#[allow(clippy::cast_lossless)]
impl const From<i32> for Solution {
    fn from(n: i32) -> Self {
        Self::Number(n as i64)
    }
}

impl const From<i64> for Solution {
    fn from(n: i64) -> Self {
        Self::Number(n)
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(n) => write!(f, "{n}"),
            Self::MerryChristmas => write!(f, "Merry Christmas!"),
            Self::Unsolved => write!(f, ""),
        }
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::MerryChristmas, Self::MerryChristmas) => true,
            _ => panic!("Tried to compare results of different types"),
        }
    }
}

pub const fn solution<T: ~const Into<Solution>>(sln: T) -> Solution {
    sln.into()
}

pub trait AocSolution {
    type Input;
    fn process_input(input: &str) -> Self::Input;

    const PART1_SOLUTION: Solution;
    const PART1_STATUS: SolutionStatus = SolutionStatus::Solved;
    fn part1(i: &Self::Input) -> impl Into<Solution>;

    const PART2_SOLUTION: Solution;
    const PART2_STATUS: SolutionStatus = SolutionStatus::Solved;
    fn part2(i: &Self::Input) -> impl Into<Solution>;
}

pub struct Unsolved;
impl AocSolution for Unsolved {
    type Input = ();
    fn process_input(_: &str) -> Self::Input {}

    const PART1_SOLUTION: Solution = Solution::Unsolved;
    const PART1_STATUS: SolutionStatus = SolutionStatus::Unsolved;
    fn part1(_: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }

    const PART2_SOLUTION: Solution = Solution::Unsolved;
    const PART2_STATUS: SolutionStatus = SolutionStatus::Unsolved;
    fn part2(_: &Self::Input) -> impl Into<Solution> {
        Solution::Unsolved
    }
}

pub trait AocYear {
    type D01: AocSolution;
    type D02: AocSolution;
    type D03: AocSolution;
    type D04: AocSolution;
    type D05: AocSolution;
    type D06: AocSolution;
    type D07: AocSolution;
    type D08: AocSolution;
    type D09: AocSolution;
    type D10: AocSolution;
    type D11: AocSolution;
    type D12: AocSolution;
    type D13: AocSolution;
    type D14: AocSolution;
    type D15: AocSolution;
    type D16: AocSolution;
    type D17: AocSolution;
    type D18: AocSolution;
    type D19: AocSolution;
    type D20: AocSolution;
    type D21: AocSolution;
    type D22: AocSolution;
    type D23: AocSolution;
    type D24: AocSolution;
    type D25: AocSolution;
}
