use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/19.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect();
    let designs = designs.lines().collect();
    (towels, designs)
}

fn design_arrangements<'a>(
    design: &'a str,
    towels: &HashSet<&str>,
    cache: &mut HashMap<&'a str, u128>,
) -> u128 {
    if design.is_empty() {
        return 1;
    }
    if cache.contains_key(design) {
        return cache[design];
    }
    let count = towels.iter().fold(0, |n, towel| {
        if design.starts_with(towel) {
            n + design_arrangements(&design[towel.len()..], towels, cache)
        } else {
            n
        }
    });
    cache.insert(design, count);
    count
}

fn part_1(input: &str) -> i32 {
    let (towels, designs) = parse_input(input);
    let mut count = 0;
    let mut cache = HashMap::new();
    for d in designs {
        let n = design_arrangements(d, &towels, &mut cache);
        if n > 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> u128 {
    let (towels, designs) = parse_input(input);
    let mut count = 0;
    let mut cache = HashMap::new();
    for d in designs {
        count += design_arrangements(d, &towels, &mut cache);
    }
    count
}

#[test]
fn test_part_1() {
    let data = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part_1(data), 6);
}

#[test]
fn test_part_2() {
    let data = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part_2(data), 16);
}
