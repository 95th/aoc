use std::collections::HashMap;

use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/01.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u128 {
    let grid = Grid::<i128>::parse(input).unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();

    for row in grid.rows() {
        a.push(row[0]);
        b.push(row[1]);
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut sum = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        sum += (a - b).abs();
    }
    sum as u128
}

fn part_2(input: &str) -> u128 {
    let grid = Grid::<u128>::parse(input).unwrap();
    let mut a = HashMap::<u128, u128>::new();
    let mut b = HashMap::<u128, u128>::new();

    for row in grid.rows() {
        *a.entry(row[0]).or_insert(0) += 1;
        *b.entry(row[1]).or_insert(0) += 1;
    }

    let mut sum = 0;
    for (k, v) in a.iter() {
        if let Some(v2) = b.get(k) {
            sum += k * v * v2;
        }
    }
    sum
}

#[test]
fn test_part_1() {
    let data = r"3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part_1(data), 11);
}

#[test]
fn test_part_2() {
    let data = r"3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part_2(data), 31);
}
