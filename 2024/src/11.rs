use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/11.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for part in input.split_ascii_whitespace() {
        *map.entry(part.parse().unwrap()).or_insert(0) += 1;
    }
    map
}

fn blink(map: &mut HashMap<usize, usize>) {
    let data = std::mem::take(map);
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

fn part_1(input: &str) -> usize {
    let mut map = parse_input(input);
    for _ in 0..25 {
        blink(&mut map);
    }
    map.values().sum()
}

fn part_2(input: &str) -> usize {
    let mut map = parse_input(input);
    for _ in 0..75 {
        blink(&mut map);
    }
    map.values().sum()
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
