#![feature(return_position_impl_trait_in_trait)]
#![feature(array_windows)]
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

use aoc::{AocYear, Unsolved};

pub struct Y2022;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;

impl AocYear for Y2022 {
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
