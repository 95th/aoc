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

fn part_2(input: &str) -> u32 {
    let secrets: Vec<u128> = input.lines().map(|s| s.parse().unwrap()).collect();
    let buyers: Vec<_> = secrets
        .iter()
        .map(|s| {
            let mut secret = *s;
            let mut prev_price = (secret % 10) as u8;
            let mut prices = vec![prev_price];
            let mut changes = vec![0];
            for _ in 0..2000 {
                secret = next_secret(secret);
                let price = (secret % 10) as u8;
                prices.push(price);
                changes.push(price as i8 - prev_price as i8);
                prev_price = price;
            }
            (prices, changes)
        })
        .collect();

    let mut change_map = HashMap::new();
    let mut seen = HashSet::new();

    for (prices, changes) in buyers.iter() {
        for i in 4..prices.len() {
            let sequence = &changes[i - 3..=i];
            if seen.insert(sequence) {
                *change_map.entry(sequence).or_default() += prices[i] as u32;
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
