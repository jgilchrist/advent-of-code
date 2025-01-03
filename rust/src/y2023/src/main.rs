#![feature(array_windows)]
#![feature(let_chains)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
#![allow(unreachable_patterns)]

use aoc::{AocYear, Unsolved};
use utils::Result;

pub struct Y2023;

mod d01;
mod d02;
mod d03;
mod d04;
// @mod@

impl AocYear for Y2023 {
    const YEAR: u32 = 2023;

    type D01 = d01::Day01;
    type D02 = d02::Day02;
    type D03 = d03::Day03;
    type D04 = d04::Day04;
    type D05 = Unsolved;
    type D06 = Unsolved;
    type D07 = Unsolved;
    type D08 = Unsolved;
    type D09 = Unsolved;
    type D10 = Unsolved;
    type D11 = Unsolved;
    type D12 = Unsolved;
    type D13 = Unsolved;
    type D14 = Unsolved;
    type D15 = Unsolved;
    type D16 = Unsolved;
    type D17 = Unsolved;
    type D18 = Unsolved;
    type D19 = Unsolved;
    type D20 = Unsolved;
    type D21 = Unsolved;
    type D22 = Unsolved;
    type D23 = Unsolved;
    type D24 = Unsolved;
    type D25 = Unsolved;
}

fn main() -> Result<()> {
    aoc::main::<Y2023>()
}
