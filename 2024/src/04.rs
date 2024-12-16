use aoc_util::{Grid, Vec2};

fn main() {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let mut count = 0;

    for p in grid.points() {
        let strings = [
            Vec2::new(0, 1),
            Vec2::new(1, 0),
            Vec2::new(1, 1),
            Vec2::new(1, -1),
        ]
        .into_iter()
        .map(|delta| {
            (0..=3)
                .map_while(|x| grid.get(p + delta * x).copied())
                .collect::<Vec<_>>()
        });

        count += strings
            .filter(|x| matches!(&x[..], b"XMAS" | b"SAMX"))
            .count();
    }

    count
}

fn part_2(input: &str) -> u32 {
    let grid = Grid::from_bytes(input);
    let mut count = 0;

    for p in grid.points() {
        let strings = [Vec2::new(-1, -1), Vec2::new(-1, 1)]
            .into_iter()
            .map(|delta| {
                (-1..=1)
                    .map_while(|x| grid.get(p + delta * x).copied())
                    .collect::<Vec<_>>()
            });

        if strings
            .filter(|x| matches!(&x[..], b"MAS" | b"SAM"))
            .count()
            == 2
        {
            count += 1;
        }
    }

    count
}

#[test]
fn test_part_1() {
    let data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part_1(data), 18);
}

#[test]
fn test_part_2() {
    let data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part_2(data), 9);
}
