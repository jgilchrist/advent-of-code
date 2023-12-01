#![feature(array_chunks)]
#![allow(incomplete_features)]

use aoc::AocYear;

pub struct Y2016;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

impl AocYear for Y2016 {
    type D01 = d01::Day01;
    type D02 = d02::Day02;
    type D03 = d03::Day03;
    type D04 = d04::Day04;
    type D05 = d05::Day05;
    type D06 = d06::Day06;
    type D07 = d07::Day07;
    type D08 = d08::Day08;
    type D09 = d09::Day09;
    type D10 = d10::Day10;
    type D11 = d11::Day11;
    type D12 = d12::Day12;
    type D13 = d13::Day13;
    type D14 = d14::Day14;
    type D15 = d15::Day15;
    type D16 = d16::Day16;
    type D17 = d17::Day17;
    type D18 = d18::Day18;
    type D19 = d19::Day19;
    type D20 = d20::Day20;
    type D21 = d21::Day21;
    type D22 = d22::Day22;
    type D23 = d23::Day23;
    type D24 = d24::Day24;
    type D25 = d25::Day25;
}
