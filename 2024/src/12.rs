use std::collections::HashSet;

use aoc_util::{Dir, Grid, Pt};

fn main() {
    let input = include_str!("../input/12.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn get_regions(grid: &Grid<u8>) -> Vec<HashSet<Pt>> {
    let mut done = HashSet::new();
    let mut regions = Vec::new();

    for point in grid.points() {
        if done.contains(&point) {
            continue;
        }

        let mut stack = vec![point];
        let mut group = HashSet::new();
        while let Some(p) = stack.pop() {
            if !done.insert(p) {
                continue;
            }
            group.insert(p);

            for n in grid.neighbors(p) {
                if grid[n] == grid[p] {
                    stack.push(n);
                }
            }
        }
        regions.push(group);
    }
    regions
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let regions = get_regions(&grid);

    let mut total = 0;
    for region in regions {
        let area = region.len();
        let perimeter: usize = region
            .iter()
            .map(|p| 4 - grid.neighbors(*p).filter(|n| grid[*n] == grid[*p]).count())
            .sum();
        total += area * perimeter;
    }
    total
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let regions = get_regions(&grid);

    let mut total = 0;
    for region in regions {
        let area = region.len();
        let sides = find_sides(&region);
        total += area * sides;
    }
    total
}

fn find_sides(region: &HashSet<Pt>) -> usize {
    let mut count = 0;
    for dir in Dir::all() {
        let mut done = HashSet::new();
        let left = dir.turn_left();
        let right = dir.turn_right();

        for p in region {
            if !done.insert(*p) {
                continue;
            }
            if region.contains(&p.step(dir)) {
                continue;
            }

            let mut point = p.step(left);
            while region.contains(&point) && !region.contains(&point.step(dir)) {
                done.insert(point);
                point = point.step(left);
            }
            let mut point = p.step(right);
            while region.contains(&point) && !region.contains(&point.step(dir)) {
                done.insert(point);
                point = point.step(right);
            }
            count += 1;
        }
    }
    count
}

#[test]
fn test_part_1() {
    let data = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    assert_eq!(part_1(data), 1930);
}

#[test]
fn test_part_1_test_2() {
    let data = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    assert_eq!(part_1(data), 772);
}

#[test]
fn test_part_1_test_3() {
    let data = r"AAAA
BBCD
BBCC
EEEC";
    assert_eq!(part_1(data), 140);
}

#[test]
fn test_part_2() {
    let data = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    assert_eq!(part_2(data), 1206);
}

#[test]
fn test_part_2_test_2() {
    let data = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    assert_eq!(part_2(data), 436);
}

#[test]
fn test_part_2_test_3() {
    let data = r"AAAA
BBCD
BBCC
EEEC";
    assert_eq!(part_2(data), 80);
}

#[test]
fn test_part_2_test_4() {
    let data = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    assert_eq!(part_2(data), 236);
}
