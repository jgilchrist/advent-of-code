use aoc::prelude::*;
use utils::prelude::*;

pub struct Day09;

// Return (idx, len) pairs denoting free space blocks on the drive
fn get_spaces(drive: &[Option<u32>]) -> Vec<(usize, usize)> {
    let mut spaces = Vec::new();

    let mut start = drive.iter().position(|p| p.is_none()).unwrap();

    'outer: loop {
        let mut end = start;

        while drive[end].is_none() {
            end += 1;

            if end == drive.len() {
                break 'outer;
            }
        }

        spaces.push((start, end - start));

        start = end + 1;

        while drive[start].is_some() {
            start += 1;

            if start == drive.len() {
                break 'outer;
            }
        }
    }

    spaces
}

fn checksum(drive: &[Option<u32>]) -> u64 {
    drive
        .iter()
        .enumerate()
        .map(|(idx, file_id)| match file_id {
            None => 0,
            Some(id) => idx as u64 * u64::from(*id),
        })
        .sum::<u64>()
}

impl AocSolution for Day09 {
    type Input = Vec<Option<u32>>;
    fn process_input(input: &str) -> Self::Input {
        let mut drive = Vec::new();

        let mut numbers = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        // Add one last fake 'empty block' so that array_chunks has something to pair the last file with
        // otherwise it will be skipped
        numbers.push(0);

        for (idx, [file_size, free_size]) in numbers.array_chunks::<2>().enumerate() {
            for _ in 0..*file_size {
                drive.push(Some(idx as u32));
            }

            for _ in 0..*free_size {
                drive.push(None);
            }
        }

        drive
    }

    const PART1_SOLUTION: SolutionStatus = solution(6344673854800i64);
    fn part1(input: &Self::Input) -> impl ToSolution {
        let mut drive = input.clone();

        let mut next_to_replace = drive.iter().position(|p| p.is_none()).unwrap();

        // The final element is always going to contain a file, so we can start at the end
        let mut next_to_shift = drive.len() - 1;

        while next_to_replace < next_to_shift {
            // Shift the last element back to the first free space
            drive[next_to_replace] = drive[next_to_shift];
            drive[next_to_shift] = None;

            // Find the next free space
            next_to_replace = drive.iter().position(|p| p.is_none()).unwrap();

            // Find the next non-empty space from the end
            next_to_shift -= 1;
            while drive[next_to_shift].is_none() {
                next_to_shift -= 1;
            }
        }

        checksum(&drive)
    }

    const PART2_SOLUTION: SolutionStatus = solution(6360363199987i64);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let mut drive = input.clone();

        let mut spaces = get_spaces(&drive);

        // The final element is always going to contain a file, so we can start at the end
        let mut end_of_file = drive.len() - 1;

        'outer: loop {
            let file_number = drive[end_of_file].unwrap();
            let mut start_of_file = end_of_file;

            while drive[start_of_file] == Some(file_number) {
                start_of_file -= 1;

                if start_of_file == 0 {
                    break 'outer;
                }
            }
            start_of_file += 1;

            let file_len = end_of_file - start_of_file + 1;

            let matching_space = spaces
                .iter()
                .find(|(idx, len)| *len >= file_len && *idx < start_of_file);

            if let Some((space_idx, _)) = matching_space {
                for i in 0..file_len {
                    drive[space_idx + i] = drive[start_of_file + i];
                    drive[start_of_file + i] = None;
                }
            }

            spaces = get_spaces(&drive);

            end_of_file = start_of_file - 1;
            while drive[end_of_file].is_none() {
                end_of_file -= 1;

                if end_of_file == 0 {
                    break 'outer;
                }
            }
        }

        checksum(&drive)
    }
}
