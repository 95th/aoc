use aoc_util::{Grid, DOWN, DOWN_LEFT, DOWN_RIGHT, RIGHT};

fn main() {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let mut count = 0;

    for p in grid.points() {
        count += [RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT]
            .into_iter()
            .map(|step| grid.get_range(p, step, 4).copied().collect::<Vec<_>>())
            .filter(|x| matches!(&x[..], b"XMAS" | b"SAMX"))
            .count();
    }

    count
}

fn part_2(input: &str) -> u32 {
    let grid = Grid::from_bytes(input);
    let mut count = 0;

    for p in grid.points() {
        let matching = [DOWN_LEFT, DOWN_RIGHT]
            .into_iter()
            .map(|step| {
                grid.get_range(p - step, step, 3)
                    .copied()
                    .collect::<Vec<_>>()
            })
            .filter(|x| matches!(&x[..], b"MAS" | b"SAM"))
            .count();

        if matching == 2 {
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
