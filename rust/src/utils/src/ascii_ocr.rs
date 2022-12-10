use std::collections::HashMap;

use itertools::Itertools;

pub fn convert(s: &str) -> String {
    let alphabet_6: HashMap<String, char> = HashMap::from([
        (
            "
.##.
#..#
#..#
####
#..#
#..#",
            'A',
        ),
        (
            "
###.
#..#
###.
#..#
#..#
###.",
            'B',
        ),
        (
            "
.##.
#..#
#...
#...
#..#
.##.",
            'C',
        ),
        (
            "
####
#...
###.
#...
#...
####",
            'E',
        ),
        (
            "
####
#...
###.
#...
#...
#...",
            'F',
        ),
        (
            "
.##.
#..#
#...
#.##
#..#
.###",
            'G',
        ),
        (
            "
#..#
#..#
####
#..#
#..#
#..#",
            'H',
        ),
        (
            "
.###
..#.
..#.
..#.
..#.
.###",
            'I',
        ),
        (
            "
..##
...#
...#
...#
#..#
.##.",
            'J',
        ),
        (
            "
#..#
#.#.
##..
#.#.
#.#.
#..#",
            'K',
        ),
        (
            "
#...
#...
#...
#...
#...
####",
            'L',
        ),
        (
            "
.##.
#..#
#..#
#..#
#..#
.##.",
            'O',
        ),
        (
            "
###.
#..#
#..#
###.
#...
#...",
            'P',
        ),
        (
            "
###.
#..#
#..#
###.
#.#.
#..#",
            'R',
        ),
        (
            "
.###
#...
#...
.##.
...#
###.",
            'S',
        ),
        (
            "
#..#
#..#
#..#
#..#
#..#
.##.",
            'U',
        ),
        (
            "
#...
#...
.#.#
..#.
..#.
..#.",
            'Y',
        ),
        (
            "
####
...#
..#.
.#..
#...
####",
            'Z',
        ),
    ])
    .iter()
    .map(|(s, mapped_char)| (s.trim().to_string(), *mapped_char))
    .collect();

    let s = s.to_string();
    let s = s.replace(' ', ".");
    let lines: Vec<&str> = s.trim().lines().collect();

    let rows = lines.len();
    let cols = lines[0].len();

    assert!(rows == 6, "Height of ASCII text must be 6 chars");
    assert!(
        !lines.iter().any(|l| l.len() != cols),
        "Lines of ASCII text were of different lengths"
    );

    let number_of_letters = cols / 5;

    let mut s: String = String::new();

    for i in 0..number_of_letters {
        let letter_start_pos = i * 5;
        let parts_of_char = lines
            .iter()
            .map(|l| l[letter_start_pos..letter_start_pos + 4].to_string())
            .join("\n");
        let this_char = alphabet_6[parts_of_char.as_str()];
        s.push(this_char);
    }

    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_conversion() {
        let s = r#"
.##..###...##..####.####..##..#..#..###...##.#..#.#.....##..###..###...###.#..#.#...#####.
#..#.#..#.#..#.#....#....#..#.#..#...#.....#.#.#..#....#..#.#..#.#..#.#....#..#.#...#...#.
#..#.###..#....###..###..#....####...#.....#.##...#....#..#.#..#.#..#.#....#..#..#.#...#..
####.#..#.#....#....#....#.##.#..#...#.....#.#.#..#....#..#.###..###...##..#..#...#...#...
#..#.#..#.#..#.#....#....#..#.#..#...#..#..#.#.#..#....#..#.#....#.#.....#.#..#...#..#....
#..#.###...##..####.#.....###.#..#..###..##..#..#.####..##..#....#..#.###...##....#..####.
"#;
        assert_eq!(super::convert(s), "ABCEFGHIJKLOPRSUYZ");
    }
}
