use prelude::*;

pub struct Day02;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

fn paper_area_required(
    &Present {
        length,
        width,
        height,
    }: &Present,
) -> u32 {
    let sides = [length * width, width * height, height * length];

    let total_area: u32 = sides.iter().map(|s| s * 2).sum();
    let extra_paper = sides.iter().min().unwrap();

    total_area + extra_paper
}

fn ribbon_length_required(
    &Present {
        length,
        width,
        height,
    }: &Present,
) -> u32 {
    let mut dimensions = [length, width, height];
    dimensions.sort_unstable();

    let [smallest, middle, _] = dimensions;

    let smallest_perimeter = (smallest + middle) * 2;
    let volume = length * width * height;

    smallest_perimeter + volume
}

impl AocSolution for Day02 {
    type Input = Vec<Present>;
    fn process_input(input: &str) -> Self::Input {
        fn parse_line(line: &str) -> Present {
            let numbers = utils::inputs::positive_numbers(line);

            let [x1, x2, x3] = numbers[..] else {
                unreachable!()
            };

            Present {
                length: x1,
                width: x2,
                height: x3,
            }
        }

        input.lines().map(parse_line).collect()
    }

    const PART1_SOLUTION: Solution = solution(1586300);
    fn part1(input: &Self::Input) -> impl Into<Solution> {
        input.iter().map(paper_area_required).sum::<u32>()
    }

    const PART2_SOLUTION: Solution = solution(3737498);
    fn part2(input: &Self::Input) -> impl Into<Solution> {
        input.iter().map(ribbon_length_required).sum::<u32>()
    }
}
