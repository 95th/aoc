use std::collections::HashMap;

pub fn blink(map: &mut HashMap<usize, usize>) {
    let data = std::mem::replace(map, HashMap::new());
    for (n, count) in data {
        if n == 0 {
            *map.entry(1).or_insert(0) += count;
        } else if (n.ilog10() + 1) % 2 == 0 {
            let digits = n.ilog10() + 1;
            let digits_pow = 10_usize.pow(digits / 2);
            *map.entry(n / digits_pow).or_insert(0) += count;
            *map.entry(n % digits_pow).or_insert(0) += count;
        } else {
            *map.entry(n * 2024).or_insert(0) += count;
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let numbers: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = HashMap::new();
    for number in numbers {
        *map.entry(number).or_insert(0) += 1;
    }

    for _ in 0..25 {
        blink(&mut map);
    }

    map.values().sum()
}

pub fn part_2(input: &str) -> usize {
    let numbers: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut map = HashMap::new();
    for number in numbers {
        *map.entry(number).or_insert(0) += 1;
    }

    for _ in 0..75 {
        blink(&mut map);
    }

    map.values().sum()
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
    let data = r"125 17";
    assert_eq!(part_1(data), 55312);
}

#[test]
fn test_part_2() {
    let data = r"125 17";
    assert_eq!(part_2(data), 65601038650482);
}
