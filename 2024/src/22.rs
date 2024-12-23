use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/22.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn next_secret(mut secret: u128) -> u128 {
    const PRUNE: u128 = 16777216;
    secret = ((secret * 64) ^ secret) % PRUNE;
    secret = ((secret / 32) ^ secret) % PRUNE;
    secret = ((secret * 2048) ^ secret) % PRUNE;
    secret
}

fn part_1(input: &str) -> u128 {
    let mut secrets: Vec<u128> = input.lines().map(|s| s.parse().unwrap()).collect();

    for secret in &mut secrets {
        let mut s = *secret;
        for _ in 0..2000 {
            s = next_secret(s);
        }
        *secret = s;
    }

    secrets.into_iter().sum()
}

fn price(secret: u128) -> i8 {
    (secret % 10) as i8
}

fn push_change(sequence: u32, change: i8) -> u32 {
    sequence << 8 | change as u8 as u32
}

fn part_2(input: &str) -> u32 {
    let mut change_map = HashMap::<u32, u32>::new();
    let mut seen = HashSet::<u32>::new();

    for mut secret in input.lines().map(|s| s.parse().unwrap()) {
        let mut prev_price = price(secret);
        let mut sequence = 0;

        for i in 0..2000 {
            secret = next_secret(secret);
            let price = price(secret);
            sequence = push_change(sequence, prev_price - price);
            prev_price = price;
            if i >= 4 && seen.insert(sequence) {
                *change_map.entry(sequence).or_default() += price as u32;
            }
        }
        seen.clear();
    }

    *change_map.values().max().unwrap()
}

#[test]
fn test_part_1() {
    let data = r"1
10
100
2024";
    assert_eq!(part_1(data), 37327623);
}

#[test]
fn test_part_2() {
    let data = r"1
2
3
2024";
    assert_eq!(part_2(data), 23);
}
