use std::{
    collections::HashMap,
    io::{BufRead, Cursor},
};

pub fn part_1(input: &str) -> u128 {
    let mut a = Vec::<i128>::new();
    let mut b = Vec::<i128>::new();

    for line in Cursor::new(input).lines() {
        let line = line.unwrap();
        let mut split = line.split_ascii_whitespace();
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        a.push(first.parse().unwrap());
        b.push(second.parse().unwrap());
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut sum = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        sum += (a - b).abs();
    }
    sum as u128
}

#[test]
fn run_part_1() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_1(input));
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

pub fn part_2(input: &str) -> u128 {
    let mut a = HashMap::<u128, u128>::new();
    let mut b = HashMap::<u128, u128>::new();

    for line in Cursor::new(input).lines() {
        let line = line.unwrap();
        let mut split = line.split_ascii_whitespace();
        let first: u128 = split.next().unwrap().parse().unwrap();
        let second: u128 = split.next().unwrap().parse().unwrap();
        *a.entry(first).or_insert(0) += 1;
        *b.entry(second).or_insert(0) += 1;
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
fn run_part_2() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_2(input));
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
