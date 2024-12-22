use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/22.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u128 {
    let mut secrets: Vec<u128> = input.lines().map(|s| s.parse().unwrap()).collect();

    for secret in &mut secrets {
        let mut s = *secret;
        for _ in 0..2000 {
            s = ((s * 64) ^ s) % 16777216;
            s = ((s / 32) ^ s) % 16777216;
            s = ((s * 2048) ^ s) % 16777216;
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
            let mut list = vec![(secret % 10) as i32];
            for _ in 0..2000 {
                secret = ((secret * 64) ^ secret) % 16777216;
                secret = ((secret / 32) ^ secret) % 16777216;
                secret = ((secret * 2048) ^ secret) % 16777216;
                list.push((secret % 10) as i32);
            }
            list
        })
        .collect();

    let change_histories: Vec<_> = prices
        .iter()
        .map(|prices| {
            let mut cache = HashMap::new();
            let mut change_history = [0; 4];
            for i in 4..prices.len() {
                for k in 0..4 {
                    change_history[k] = prices[i - 4 + k + 1] - prices[i - 4 + k];
                }
                if !cache.contains_key(&change_history) {
                    cache.insert(change_history, prices[i]);
                }
            }
            cache
        })
        .collect();

    let mut combined = HashMap::<[i32; 4], u128>::new();
    for change_history in change_histories
        .iter()
        .flat_map(|m| m.keys())
        .collect::<HashSet<_>>()
    {
        for i in 0..prices.len() {
            *combined.entry(*change_history).or_default() += change_histories[i]
                .get(change_history)
                .copied()
                .unwrap_or(0) as u128;
        }
    }

    *combined.values().max().unwrap()
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
