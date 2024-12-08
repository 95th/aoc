use std::collections::HashMap;

use crate::util::Matrix;

pub fn part_1(input: &str) -> u32 {
    let grid = Matrix::from_bytes(input);

    let mut map = HashMap::new();

    for point in grid.iter_points() {
        if grid[point] != b'.' {
            map.entry(grid[point]).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = grid.map(false);
    let mut count = 0;

    for points in map.values() {
        for i in 0..points.len() {
            let a = points[i];
            for j in i + 1..points.len() {
                let b = points[j];
                let dist = b - a;
                let before = a - dist;
                let after = b + dist;

                if let Some(false) = antinodes.replace(before, true) {
                    count += 1;
                }
                if let Some(false) = antinodes.replace(after, true) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_2(input: &str) -> u32 {
    let grid = Matrix::from_bytes(input);

    let mut map = HashMap::new();

    for point in grid.iter_points() {
        if grid[point] != b'.' {
            map.entry(grid[point]).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = grid.map(false);
    let mut count = 0;

    for points in map.values() {
        for i in 0..points.len() {
            let a = points[i];
            for j in i + 1..points.len() {
                let b = points[j];
                let dist = b - a;

                let mut before = a;
                while let Some(set) = antinodes.replace(before, true) {
                    before -= dist;
                    if !set {
                        count += 1;
                    }
                }

                let mut after = b;
                while let Some(set) = antinodes.replace(after, true) {
                    after += dist;
                    if !set {
                        count += 1;
                    }
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
    let data = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part_1(data), 14);
}

#[test]
fn test_part_2() {
    let data = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part_2(data), 34);
}
