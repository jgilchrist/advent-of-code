#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(let_chains)]
#![allow(incomplete_features)]

mod progress;
mod runner;

pub use self::progress::print_year_progress;
pub use self::runner::main;

pub enum SolutionStatus {
    Solved(Solution),
    SolvedInPython(Solution),
    Wip,
    Unsolved,
}

pub enum Solution {
    Number(i64),
    String(&'static str),
    MerryChristmas,
    Unsolved,
}

#[const_trait]
pub trait ToSolution {
    fn to_solution(self) -> Solution;
}

impl const ToSolution for Solution {
    fn to_solution(self) -> Solution {
        self
    }
}

impl const ToSolution for &'static str {
    fn to_solution(self) -> Solution {
        Solution::String(self)
    }
}

impl ToSolution for String {
    fn to_solution(self) -> Solution {
        // Used to be able to compare runtime-generated String results with
        // static strings embedded in the binary. There's likely a better
        // way to do this, but I haven't found it.
        // This is fine since the process lives only long enough to run the solution.
        let leaked_string = Box::leak(self.into_boxed_str());
        Solution::String(leaked_string)
    }
}

impl const ToSolution for u16 {
    fn to_solution(self) -> Solution {
        Solution::Number(self as i64)
    }
}

impl const ToSolution for u32 {
    fn to_solution(self) -> Solution {
        Solution::Number(self as i64)
    }
}

impl const ToSolution for u64 {
    fn to_solution(self) -> Solution {
        Solution::Number(self as i64)
    }
}

impl const ToSolution for usize {
    fn to_solution(self) -> Solution {
        Solution::Number(self as i64)
    }
}

impl const ToSolution for i32 {
    fn to_solution(self) -> Solution {
        Solution::Number(self as i64)
    }
}

impl const ToSolution for i64 {
    fn to_solution(self) -> Solution {
        Solution::Number(self)
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

pub const fn solution<T: ~const ToSolution>(sln: T) -> SolutionStatus {
    SolutionStatus::Solved(sln.to_solution())
}

pub const fn solution_from_python<T: ~const ToSolution>(sln: T) -> SolutionStatus {
    SolutionStatus::SolvedInPython(sln.to_solution())
}

pub trait AocSolution {
    const IS_SOLVED: bool = true;

    type Input;
    fn process_input(input: &str) -> Self::Input;

    const PART1_SOLUTION: SolutionStatus;
    fn part1(i: &Self::Input) -> impl ToSolution;

    const PART2_SOLUTION: SolutionStatus;
    fn part2(i: &Self::Input) -> impl ToSolution;
}

pub struct Unsolved;
impl AocSolution for Unsolved {
    const IS_SOLVED: bool = false;

    type Input = ();
    fn process_input(_: &str) -> Self::Input {}

    const PART1_SOLUTION: SolutionStatus = SolutionStatus::Unsolved;
    fn part1((): &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }

    const PART2_SOLUTION: SolutionStatus = SolutionStatus::Unsolved;
    fn part2((): &Self::Input) -> impl ToSolution {
        Solution::Unsolved
    }
}

pub trait AocYear {
    const YEAR: u32;

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
