use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|s| s.as_bytes().to_vec()).collect()
}

fn direction(guard: u8) -> (isize, isize) {
    match guard {
        b'^' => (-1, 0),
        b'v' => (1, 0),
        b'<' => (0, -1),
        b'>' => (0, 1),
        _ => unreachable!(),
    }
}

fn turn_right(guard: u8) -> u8 {
    match guard {
        b'^' => b'>',
        b'v' => b'<',
        b'<' => b'^',
        b'>' => b'v',
        _ => unreachable!(),
    }
}

fn guard_position(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'^' || cell == b'v' || cell == b'<' || cell == b'>' {
                return (i, j);
            }
        }
    }

    unreachable!()
}

pub fn part_1(input: &str) -> usize {
    let grid = parse_input(input);
    let (x, y) = guard_position(&grid);

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut i = x as isize;
    let mut j = y as isize;

    let mut guard = grid[x][y];
    let mut dir = direction(guard);
    let mut points = HashSet::<(isize, isize)>::new();

    while i >= 0 && i < cols && j >= 0 && j < rows {
        if grid[i as usize][j as usize] == b'#' {
            i -= dir.0;
            j -= dir.1;
            guard = turn_right(guard);
            dir = direction(guard);
        } else {
            points.insert((i, j));
        }
        i += dir.0;
        j += dir.1;
    }

    points.len()
}

pub fn is_loop(grid: Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut i = x as isize;
    let mut j = y as isize;

    let mut guard = grid[x][y];
    let mut dir = direction(guard);
    let mut points = HashSet::<(isize, isize)>::new();
    let mut points_with_dir = HashSet::<(isize, isize, char)>::new();

    while i >= 0 && i < cols && j >= 0 && j < rows {
        if grid[i as usize][j as usize] == b'#' {
            i -= dir.0;
            j -= dir.1;
            guard = turn_right(guard);
            dir = direction(guard);
        } else {
            points.insert((i, j));
            if !points_with_dir.insert((i, j, guard as char)) {
                return true;
            }
        }
        i += dir.0;
        j += dir.1;
    }

    false
}

pub fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let (x, y) = guard_position(&grid);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if i == x && j == y {
                continue;
            }
            if grid[i][j] == b'.' {
                let mut grid = grid.clone();
                grid[i][j] = b'#';
                if is_loop(grid, x, y) {
                    count += 1;
                }
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
    let data = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part_1(data), 41);
}

#[test]
fn test_part_2() {
    let data = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part_2(data), 6);
}
