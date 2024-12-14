fn main() {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..rows {
        for j in 0..cols {
            let strings = [(0, 1), (1, 0), (1, 1), (1, -1)].iter().map(|(x, y)| {
                (0..=3)
                    .map_while(|p| {
                        let i = (i + p * x) as usize;
                        let j = (j + p * y) as usize;
                        grid.get(i).and_then(|row| row.get(j)).copied()
                    })
                    .collect::<Vec<_>>()
            });

            count += strings
                .filter(|x| matches!(&x[..], b"XMAS" | b"SAMX"))
                .count();
        }
    }

    count
}

fn part_2(input: &str) -> u32 {
    let grid = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..rows {
        for j in 0..cols {
            let strings = [(1, 1), (1, -1)].iter().map(|(x, y)| {
                (-1..=1)
                    .map_while(|p| {
                        let i = (i + p * x) as usize;
                        let j = (j + p * y) as usize;
                        grid.get(i).and_then(|row| row.get(j)).copied()
                    })
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
