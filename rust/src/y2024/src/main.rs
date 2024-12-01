#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use anyhow::Result;
use aoc::{AocYear, Unsolved};

pub struct Y2024;

mod d01;
// @mod@

impl AocYear for Y2024 {
    const YEAR: u32 = 2024;

    type D01 = d01::Day01;
    type D02 = Unsolved;
    type D03 = Unsolved;
    type D04 = Unsolved;
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
    aoc::main::<Y2024>()
}
