use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/22.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn next_secret(mut value: u128) -> u128 {
    const PRUNE: u128 = 16777216;
    value = ((value * 64) ^ value) % PRUNE;
    value = ((value / 32) ^ value) % PRUNE;
    value = ((value * 2048) ^ value) % PRUNE;
    value
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

fn part_2(input: &str) -> u128 {
    let secrets: Vec<u128> = input.lines().map(|s| s.parse().unwrap()).collect();
    let prices: Vec<_> = secrets
        .iter()
        .map(|s| {
            let mut secret = *s;
            let mut history = vec![(secret % 10) as i32];
            for _ in 0..2000 {
                secret = next_secret(secret);
                history.push((secret % 10) as i32);
            }
            history
        })
        .collect();

    let mut price_change_map = HashMap::new();

    for prices in prices {
        let mut cache = HashSet::new();
        for i in 4..prices.len() {
            let change_sequence = [
                prices[i - 3] - prices[i - 4],
                prices[i - 2] - prices[i - 3],
                prices[i - 1] - prices[i - 2],
                prices[i] - prices[i - 1],
            ];
            if cache.insert(change_sequence) {
                *price_change_map.entry(change_sequence).or_default() += prices[i] as u128;
            }
        }
    }

    *price_change_map.values().max().unwrap()
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
