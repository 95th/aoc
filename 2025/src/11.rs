use std::collections::HashMap;

use aoc_util::Parse;

fn main() {
    let input = include_str!("../input/11.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse(input: &str) -> HashMap<&str, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").unwrap();
            let values = value.list(" ");
            (key, values)
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let mappings = parse(input);
    let mut path_counts = HashMap::new();
    part_1_helper(&mappings, &mut path_counts, "you")
}

fn part_1_helper<'a>(
    mappings: &'a HashMap<&str, Vec<String>>,
    path_counts: &mut HashMap<&'a str, usize>,
    key: &'a str,
) -> usize {
    if key == "out" {
        return 1;
    }

    if let Some(&count) = path_counts.get(key) {
        return count;
    }

    let count = mappings
        .get(key)
        .iter()
        .flat_map(|values| values.iter())
        .map(|next| part_1_helper(mappings, path_counts, next))
        .sum();
    path_counts.insert(key, count);
    count
}

fn part_2(input: &str) -> usize {
    let mappings = parse(input);
    let mut path_counts = HashMap::new();
    part_2_helper(&mappings, &mut path_counts, "svr", false, false)
}

fn part_2_helper<'a>(
    mappings: &'a HashMap<&str, Vec<String>>,
    path_counts: &mut HashMap<(&'a str, bool, bool), usize>,
    key: &'a str,
    seen_fft: bool,
    seen_dac: bool,
) -> usize {
    if key == "out" {
        return if seen_fft && seen_dac { 1 } else { 0 };
    }

    if let Some(&count) = path_counts.get(&(key, seen_fft, seen_dac)) {
        return count;
    }

    let count = mappings
        .get(key)
        .iter()
        .flat_map(|values| values.iter())
        .map(|next| {
            part_2_helper(
                mappings,
                path_counts,
                next,
                seen_fft || next == "fft",
                seen_dac || next == "dac",
            )
        })
        .sum();
    path_counts.insert((key, seen_fft, seen_dac), count);
    count
}

#[test]
fn test_part_1() {
    let data = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    assert_eq!(part_1(data), 5);
}

#[test]
fn test_part_2() {
    let data = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    assert_eq!(part_2(data), 2);
}
