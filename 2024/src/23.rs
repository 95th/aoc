use std::collections::HashSet;

use aoc_util::BiGraph;

fn main() {
    let input = include_str!("../input/23.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let connections: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .collect();

    let mut graph = BiGraph::new();
    for (a, b) in connections {
        graph.add_edge(a, b);
    }

    let mut set = HashSet::new();
    for node in graph.vertices() {
        if !node.starts_with('t') {
            continue;
        }
        for first in graph.neighbors(node) {
            for second in graph.neighbors(first) {
                if second != node && graph.contains_edge(node, second) {
                    let mut data = [node, first, second];
                    data.sort();
                    set.insert(data);
                }
            }
        }
    }
    set.len()
}

/// Bron-Kerbosch algorithm
fn find_max_clique<'a>(
    graph: &BiGraph<&str>,
    r: Vec<&'a str>,
    mut p: Vec<&'a str>,
    mut x: HashSet<&'a str>,
    max_clique: &mut Vec<&'a str>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r;
        }
        return;
    }

    while let Some(v) = p.pop() {
        {
            let mut r = r.clone();
            let mut p = p.clone();
            let mut x = x.clone();
            r.push(v);
            p.retain(|&n| graph.contains_edge(v, n));
            x.retain(|&n| graph.contains_edge(v, n));
            find_max_clique(graph, r, p, x, max_clique);
        }
        x.insert(v);
    }
}

fn part_2(input: &str) -> String {
    let connections: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .collect();

    let mut graph = BiGraph::new();
    for (a, b) in connections {
        graph.add_edge(a, b);
    }

    let mut max_clique = Vec::new();
    find_max_clique(
        &graph,
        Vec::new(),
        graph.vertices().copied().collect(),
        HashSet::new(),
        &mut max_clique,
    );

    max_clique.sort();
    max_clique.join(",")
}

#[test]
fn test_part_1() {
    let data = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    assert_eq!(part_1(data), 7);
}

#[test]
fn test_part_2() {
    let data = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    assert_eq!(part_2(data), "co,de,ka,ta");
}
