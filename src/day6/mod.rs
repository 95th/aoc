use std::collections::HashSet;

use crate::util::{Direction, Matrix, Point};

fn direction(guard: u8) -> Direction {
    match guard {
        b'^' => Direction::Up,
        b'v' => Direction::Down,
        b'<' => Direction::Left,
        b'>' => Direction::Right,
        _ => unreachable!(),
    }
}

fn guard_position(grid: &Matrix) -> Point {
    grid.find(|c| matches!(c, b'^' | b'v' | b'<' | b'>'))
        .unwrap()
}

pub fn part_1(input: &str) -> usize {
    let grid = Matrix::parse(input);
    let start = guard_position(&grid);
    let mut dir = direction(grid[start]);

    let mut current = start;
    let mut points = HashSet::<Point>::new();

    while grid.get(current).is_some() {
        let next = current.step(dir);
        if grid.get(next) == Some(b'#') {
            dir = dir.turn_right();
        } else {
            current = next;
            if !points.insert(current) {
                break;
            }
        }
    }

    points.len()
}

pub fn is_loop(grid: &Matrix, start: Point) -> bool {
    let mut dir = direction(grid[start]);
    let mut points_with_dir = HashSet::<(Point, Direction)>::new();

    let mut current = start;
    while grid.get(current).is_some() {
        let next = current.step(dir);
        if grid.get(next) == Some(b'#') {
            dir = dir.turn_right();
        } else {
            current = next;
            if !points_with_dir.insert((current, dir)) {
                return true;
            }
        }
    }

    false
}

pub fn part_2(input: &str) -> usize {
    let mut grid = Matrix::parse(input);
    let start = guard_position(&grid);

    let mut count = 0;
    for point in grid.iter_points() {
        if point == start || grid[point] == b'#' {
            continue;
        }
        grid[point] = b'#';
        if is_loop(&grid, start) {
            count += 1;
        }
        grid[point] = b'.';
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
