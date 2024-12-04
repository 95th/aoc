fn is_match_at_index(
    grid: &[&[u8]],
    search: &[u8],
    mut i: isize,
    mut j: isize,
    dir_i: isize,
    dir_j: isize,
) -> bool {
    let mut c = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    while c < search.len() && i >= 0 && i < rows && j >= 0 && j < cols {
        if grid[i as usize][j as usize] != search[c] {
            return false;
        }
        i += dir_i;
        j += dir_j;
        c += 1;
    }

    c == search.len()
}

pub fn part_1(input: &str) -> u32 {
    let grid = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    for i in 0..rows {
        for j in 0..cols {
            if is_match_at_index(&grid, b"XMAS", i, j, 1, 0) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, 0, 1) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, 1, 1) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, 1, -1) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, -1, 0) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, 0, -1) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, -1, -1) {
                count += 1;
            }
            if is_match_at_index(&grid, b"XMAS", i, j, -1, 1) {
                count += 1;
            }
        }
    }

    count
}

fn is_match_at_index_part_2(grid: &[&[u8]], i: usize, j: usize) -> bool {
    let i = i as isize;
    let j = j as isize;
    let tl = is_match_at_index(grid, b"MAS", i - 1, j - 1, 1, 1);
    let tr = is_match_at_index(grid, b"MAS", i - 1, j + 1, 1, -1);
    let bl = is_match_at_index(grid, b"MAS", i + 1, j - 1, -1, 1);
    let br = is_match_at_index(grid, b"MAS", i + 1, j + 1, -1, -1);
    tl && tr || tr && br || br && bl || bl && tl
}

pub fn part_2(input: &str) -> u32 {
    let grid = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if is_match_at_index_part_2(&grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn run_part_1() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_1(input));
}

#[test]
fn run_part_2() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_2(input));
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
