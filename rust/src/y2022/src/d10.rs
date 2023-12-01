use prelude::*;
use utils::ascii_ocr;

pub struct Day10;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

fn get_all_xs(instructions: &[Instruction]) -> Vec<i32> {
    let mut x = 1;
    let mut xs: Vec<i32> = Vec::new();

    for instruction in instructions {
        match instruction {
            Instruction::Noop => xs.push(x),
            Instruction::Addx(n) => {
                xs.push(x);
                xs.push(x);
                x += n;
            }
        }
    }

    xs
}

impl AocSolution for Day10 {
    type Input = Vec<Instruction>;
    fn process_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                if l == "noop" {
                    Instruction::Noop
                } else {
                    let [n] = inputs::n_numbers(l);
                    Instruction::Addx(n)
                }
            })
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(14320);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let xs = get_all_xs(input);

        [20usize, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|n| xs[n - 1] * n as i32)
            .sum::<i32>()
    }

    const PART2_SOLUTION: Solution = solution("PCPBKAPJ");
    #[allow(clippy::cast_sign_loss)]
    fn part2(input: &Self::Input) -> impl ToSolution {
        let xs = get_all_xs(input);

        let line_width = 40;

        let pixels = (0..240)
            .map(|idx| {
                let sprite_position = xs[idx] % line_width;
                (sprite_position - 1..=sprite_position + 1).contains(&(idx as i32 % line_width))
            })
            .collect_vec();

        let mut output = "\n".to_owned();

        for y in 0..6 {
            for x in 0..line_width {
                output += if pixels[y * line_width as usize + x as usize] {
                    "#"
                } else {
                    " "
                }
            }
            output += "\n";
        }

        ascii_ocr::convert(&output)
    }
}
