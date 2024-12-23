use std::collections::{HashMap, HashSet};

use petgraph::graph::{NodeIndex, UnGraph};

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

    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::<&str, NodeIndex>::new();
    for (a, b) in connections {
        let a = *node_map.entry(a).or_insert_with(|| graph.add_node(a));
        let b = *node_map.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(a, b, ());
    }

    let mut set = HashSet::new();
    for (k, node) in node_map {
        if !k.starts_with('t') {
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

fn part_2(input: &str) -> String {
    let connections: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .collect();

    let mut graph = UnGraph::new_undirected();
    let mut node_map = HashMap::<&str, NodeIndex>::new();
    for (a, b) in connections {
        let a = *node_map.entry(a).or_insert_with(|| graph.add_node(a));
        let b = *node_map.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(a, b, ());
    }

    let mut connected_sets = Vec::<HashSet<NodeIndex>>::new();
    for &node in node_map.values() {
        let mut new_connected_sets = Vec::new();
        let mut found = false;
        for set in &mut connected_sets {
            let mut connected: HashSet<NodeIndex> = set
                .iter()
                .filter(|x| graph.contains_edge(**x, node))
                .copied()
                .collect();

            if connected.len() == set.len() {
                set.insert(node);
                found = true;
            } else if !connected.is_empty() {
                connected.insert(node);
                new_connected_sets.push(connected);
                found = true;
            }
        }
        if !found {
            new_connected_sets.push(HashSet::from([node]));
        }
        connected_sets.extend(new_connected_sets);
    }

    connected_sets.sort_by_key(|x| x.len());
    let mut lan_party = connected_sets
        .pop()
        .unwrap()
        .into_iter()
        .map(|node| *graph.node_weight(node).unwrap())
        .collect::<Vec<_>>();
    lan_party.sort();
    lan_party.join(",")
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
