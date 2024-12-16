#![feature(const_trait_impl)]
#![feature(array_windows)]
#![feature(array_chunks)]
#![allow(incomplete_features)]

use anyhow::Result;
use aoc::{AocYear, Unsolved};

pub struct Y2024;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
// @mod@

impl AocYear for Y2024 {
    const YEAR: u32 = 2024;

    type D01 = d01::Day01;
    type D02 = d02::Day02;
    type D03 = d03::Day03;
    type D04 = d04::Day04;
    type D05 = d05::Day05;
    type D06 = d06::Day06;
    type D07 = d07::Day07;
    type D08 = d08::Day08;
    type D09 = d09::Day09;
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
