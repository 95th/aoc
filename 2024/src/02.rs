use std::cmp::Ordering;

use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/02.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn is_safe(levels: &[i32]) -> bool {
    let mut increasing = 0;
    let mut decreasing = 0;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        match diff.cmp(&0) {
            Ordering::Greater => increasing += 1,
            Ordering::Less => decreasing += 1,
            Ordering::Equal => (),
        }
    }
    increasing == levels.len() - 1 || decreasing == levels.len() - 1
}

fn is_almost_safe(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut new_levels = levels.to_vec();
        new_levels.remove(i);
        if is_safe(&new_levels) {
            return true;
        }
    }

    false
}

fn part_1(input: &str) -> usize {
    let grid = Grid::parse(input).unwrap();
    let mut safe = 0;
    for levels in grid.rows() {
        if is_safe(levels) {
            safe += 1;
        }
    }
    safe
}

fn part_2(input: &str) -> usize {
    let grid = Grid::parse(input).unwrap();
    let mut safe = 0;
    for levels in grid.rows() {
        if is_almost_safe(levels) {
            safe += 1;
        }
    }
    safe
}

#[test]
fn test_part_1() {
    let data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part_1(data), 2);
}

#[test]
fn test_part_2() {
    let data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part_2(data), 4);
}

#[test]
fn test_part_2_1() {
    let data = r"1 3 2 4 50";
    assert_eq!(part_2(data), 0);
}

#[test]
fn test_part_2_2() {
    let data = r"10 1 2 4 7";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_3() {
    let data = r"4 10 9 8 7";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_4() {
    let data = r"4 4 5 6 7";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_5() {
    let data = r"4 4 3 2 1";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_6() {
    let data = r"1 0 4 5 6";
    assert_eq!(part_2(data), 1);
}
