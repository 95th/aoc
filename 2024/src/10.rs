use std::collections::HashSet;

use aoc_util::{Grid, Vec2};

fn main() {
    let input = include_str!("../input/10.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn find_trail(grid: &Grid<u8>, point: Vec2, value: u8, destinations: &mut HashSet<Vec2>) -> usize {
    if grid[point] == b'9' {
        destinations.insert(point);
        return 1;
    }

    grid.neighbors(point).fold(0, |count, next| {
        if grid[next] == value {
            count + find_trail(grid, next, value + 1, destinations)
        } else {
            count
        }
    })
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);

    let mut sum = 0;
    let destinations = &mut HashSet::new();

    for point in grid.points() {
        if grid[point] == b'0' {
            find_trail(&grid, point, b'1', destinations);
            sum += destinations.len();
            destinations.clear();
        }
    }

    sum
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_bytes(input);

    let mut sum = 0;
    let destinations = &mut HashSet::new();

    for point in grid.points() {
        if grid[point] == b'0' {
            sum += find_trail(&grid, point, b'1', destinations);
            destinations.clear();
        }
    }

    sum
}

#[test]
fn test_part_1() {
    let data = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part_1(data), 36);
}

#[test]
fn test_part_2() {
    let data = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part_2(data), 81);
}
