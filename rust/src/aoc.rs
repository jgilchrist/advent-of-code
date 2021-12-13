pub trait AocSolution<const N: u32> {
    type Input;
    type Output: std::fmt::Display + Eq;

    const SOLVED: bool = true;

    fn get_input() -> Self::Input;

    const PART1_SOLUTION: Option<Self::Output>;
    fn part1(i: &Self::Input) -> Self::Output;

    const PART2_SOLUTION: Option<Self::Output>;
    fn part2(i: &Self::Input) -> Self::Output;
}

pub struct Unsolved<const N: u32>;
impl<const N: u32> AocSolution<N> for Unsolved<N> {
    type Input = ();
    type Output = &'static str;

    const SOLVED: bool = false;

    fn get_input() -> Self::Input {
        unimplemented!()
    }

    const PART1_SOLUTION: Option<Self::Output> = None;
    fn part1(_: &Self::Input) -> Self::Output {
        unimplemented!()
    }

    const PART2_SOLUTION: Option<Self::Output> = None;
    fn part2(_: &Self::Input) -> Self::Output {
        unimplemented!()
    }
}

pub trait AocYear<const NYEAR: u32> {
    type D01: AocSolution<1>;
    type D02: AocSolution<2>;
    type D03: AocSolution<3>;
    type D04: AocSolution<4>;
    type D05: AocSolution<5>;
    type D06: AocSolution<6>;
    type D07: AocSolution<7>;
    type D08: AocSolution<8>;
    type D09: AocSolution<9>;
    type D10: AocSolution<10>;
    type D11: AocSolution<11>;
    type D12: AocSolution<12>;
    type D13: AocSolution<13>;
    type D14: AocSolution<14>;
    type D15: AocSolution<15>;
    type D16: AocSolution<16>;
    type D17: AocSolution<17>;
    type D18: AocSolution<18>;
    type D19: AocSolution<19>;
    type D20: AocSolution<20>;
    type D21: AocSolution<21>;
    type D22: AocSolution<22>;
    type D23: AocSolution<23>;
    type D24: AocSolution<24>;
    type D25: AocSolution<25>;
}
