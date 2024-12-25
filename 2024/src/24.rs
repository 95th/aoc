use std::collections::{HashMap, HashSet, VecDeque};

use aoc_util::DiGraph;

fn main() {
    let input = include_str!("../input/24.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> (Vec<(&str, u8)>, Vec<(&str, &str, &str, &str)>) {
    let (initial_values, gates) = input.split_once("\n\n").unwrap();
    let initial_values = initial_values
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            (name, value.parse().unwrap())
        })
        .collect();
    let gates = gates
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let (lhs1, op_lhs2) = lhs.split_once(" ").unwrap();
            let (op, lhs2) = op_lhs2.split_once(" ").unwrap();
            (lhs1, op, lhs2, rhs)
        })
        .collect();
    (initial_values, gates)
}

fn part_1(input: &str) -> u64 {
    let (initial_values, mut gates) = parse_input(input);

    let mut graph = DiGraph::new();
    for (a, _op, b, out) in gates.iter() {
        graph.add_edge(*a, *out);
        graph.add_edge(*b, *out);
    }

    let sorted_wires = graph.topological_sort();
    gates.sort_by_key(|(_a, _op, _b, out)| {
        sorted_wires.len() - sorted_wires.iter().position(|&x| x == *out).unwrap()
    });

    let mut values = HashMap::new();
    for (name, value) in initial_values {
        values.insert(name, value);
    }

    for (a, op, b, out) in gates {
        let a_value = values[a];
        let b_value = values[b];
        match op {
            "AND" => values.insert(out, a_value & b_value),
            "OR" => values.insert(out, a_value | b_value),
            "XOR" => values.insert(out, a_value ^ b_value),
            _ => unreachable!("INVALID GATE"),
        };
    }

    let mut out: u64 = 0;

    for i in 0..64 {
        let key = &format!("z{:02}", i)[..];
        if values.contains_key(key) {
            out |= (values[key] as u64) << i;
        }
    }

    out
}

fn part_2(input: &str) -> String {
    let (initial_values, mut gates) = parse_input(input);

    let replacements = [
        ("gjc", "qjj"),
        ("qsb", "z39"),
        ("wmp", "z17"),
        ("gvm", "z26"),
    ];

    for (a, b) in replacements {
        for (.., rhs) in gates.iter_mut() {
            if *rhs == a {
                *rhs = b;
            } else if *rhs == b {
                *rhs = a;
            }
        }
    }

    let mut graph = DiGraph::new();
    for (a, _op, b, out) in gates.iter() {
        graph.add_edge(*a, *out);
        graph.add_edge(*b, *out);
    }

    let sorted_wires = graph.topological_sort();
    gates.sort_by_key(|(_a, _op, _b, out)| {
        sorted_wires.len() - sorted_wires.iter().position(|&x| x == *out).unwrap()
    });

    let mut map = HashMap::new();
    for (name, _) in initial_values.iter().copied() {
        map.insert(name, None);
    }

    for (a, op, b, out) in gates.iter().copied() {
        map.insert(out, Some((a, op, b)));
    }

    let mut seen = HashSet::new();
    for i in 0..46 {
        let key = &format!("z{:02}", i)[..];
        println!("\nkey: {key}");
        let mut pending = VecDeque::new();
        pending.push_back((key, 0));

        while let Some((key, d)) = pending.pop_front() {
            if !seen.insert(key.to_string()) {
                continue;
            }
            match map.get(key).unwrap() {
                Some((a, op, b)) => {
                    println!("{:01$} {key} => {a} {op} {b}", ' ', 2 * d);
                    pending.push_back((a, d + 1));
                    pending.push_back((b, d + 1));
                }
                None => (),
            }
        }
    }

    let mut values = HashMap::new();
    for (name, value) in initial_values {
        values.insert(name, value);
    }

    for (a, op, b, out) in gates {
        let a_value = values[a];
        let b_value = values[b];
        match op {
            "AND" => values.insert(out, a_value & b_value),
            "OR" => values.insert(out, a_value | b_value),
            "XOR" => values.insert(out, a_value ^ b_value),
            _ => unreachable!("INVALID GATE"),
        };
    }

    let mut z: u64 = 0;
    for i in 0..46 {
        let key = &format!("z{:02}", i)[..];
        if values.contains_key(key) {
            z |= (values[key] as u64) << i;
        }
    }

    let mut x: u64 = 0;
    for i in 0..46 {
        let key = &format!("x{:02}", i)[..];
        if values.contains_key(key) {
            x |= (values[key] as u64) << i;
        }
    }

    let mut y: u64 = 0;
    for i in 0..46 {
        let key = &format!("y{:02}", i)[..];
        if values.contains_key(key) {
            y |= (values[key] as u64) << i;
        }
    }

    println!("{x} + {y} = {z}");

    // Found manually by printing the wire connections above.
    let replacements = [
        ("gjc", "qjj"),
        ("qsb", "z39"),
        ("wmp", "z17"),
        ("gvm", "z26"),
    ];
    let mut replacements: Vec<_> = replacements.into_iter().flat_map(|x| [x.0, x.1]).collect();
    replacements.sort();
    replacements.join(",")
}

#[test]
fn test_part_1() {
    let data = r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    assert_eq!(part_1(data), 4);
}

#[test]
fn test_part_1_larger() {
    let data = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    assert_eq!(part_1(data), 2024);
}
