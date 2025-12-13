use std::f64;

use aoc_util::Parse;
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};
use regex::Regex;

fn main() {
    let input = include_str!("../input/10.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse(input: &str) -> Vec<(u32, Vec<u32>, Vec<u32>)> {
    let lights_regex = Regex::new(r"\[(.*)\]").unwrap();
    let switch_regex = Regex::new(r"(\((.*?)\))+").unwrap();
    let joltage_regex = Regex::new(r"\{(.*)\}").unwrap();
    input
        .lines()
        .map(|line| {
            let lights = lights_regex
                .captures(line)
                .iter()
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .enumerate()
                .fold(
                    0,
                    |prev, (i, c)| if c == '#' { prev | (1 << i) } else { prev },
                );
            let switches = switch_regex
                .captures_iter(line)
                .map(|m| {
                    m.get(2)
                        .unwrap()
                        .as_str()
                        .list::<u32>(",")
                        .into_iter()
                        .fold(0, |prev, x| prev | (1 << x))
                })
                .collect();
            let joltage = joltage_regex
                .captures(line)
                .iter()
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .list(",");

            (lights, switches, joltage)
        })
        .collect()
}

fn part_1(input: &str) -> u32 {
    let mut output = 0;
    for (lights, switches, _) in parse(input) {
        let mut min_count = u32::MAX;

        for i in 0_u32..(1 << switches.len()) {
            let mut current_lights = 0;
            for (j, switch) in switches.iter().enumerate() {
                if i & (1 << j) != 0 {
                    current_lights ^= switch;
                }
            }
            if current_lights == lights {
                min_count = min_count.min(i.count_ones());
            }
        }

        output += min_count;
    }
    output
}

fn part_2(input: &str) -> f64 {
    let mut output = 0.0;

    for (_, switches, joltage) in parse(input) {
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let mut vars = Vec::new();
        for _ in 0..switches.len() {
            let a = problem.add_integer_var(1.0, (0, i32::MAX));
            vars.push(a);
        }

        for i in 0..joltage.len() {
            let mut expr = LinearExpr::empty();
            for (j, switch) in switches.iter().enumerate() {
                if switch & (1 << i) != 0 {
                    expr.add(vars[j], 1.0);
                }
            }
            problem.add_constraint(expr, ComparisonOp::Eq, joltage[i] as f64);
        }

        let solution = problem.solve().unwrap();
        output += solution.objective();
    }

    output
}

#[test]
fn test_part_1() {
    let data = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(part_1(data), 7);
}

#[test]
fn test_part_2() {
    let data = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(part_2(data), 33.0);
}
