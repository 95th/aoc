use std::collections::{HashMap, HashSet};

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

    let mut map = HashMap::new();

    for (a, b) in connections {
        map.entry(a).or_insert_with(HashSet::new).insert(b);
        map.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut set = HashSet::new();

    for (k, v) in &map {
        if !k.starts_with('t') {
            continue;
        }

        for x in v {
            if x == k {
                continue;
            }

            for y in &map[x] {
                if y == k {
                    continue;
                }

                if map[y].contains(k) {
                    let mut data = [*k, *x, *y];
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

    let mut map = HashMap::new();

    for (a, b) in connections {
        map.entry(a).or_insert_with(HashSet::new).insert(b);
        map.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut connected_sets = Vec::<HashSet<&str>>::new();
    for &k in map.keys() {
        let mut new_connected_sets = Vec::new();
        let mut found = false;
        for set in &mut connected_sets {
            if set.iter().all(|x| map[x].contains(k)) {
                set.insert(k);
            } else {
                let mut set: HashSet<_> = set
                    .iter()
                    .filter(|&x| map[x].contains(k))
                    .copied()
                    .collect();
                if !set.is_empty() {
                    set.insert(k);
                    new_connected_sets.push(set);
                    found = true;
                }
            }
        }
        if !found {
            new_connected_sets.push(HashSet::from([k]));
        }
        connected_sets.extend(new_connected_sets);
    }

    connected_sets.sort_by_key(|x| x.len());
    let mut lan_party = connected_sets
        .pop()
        .unwrap()
        .into_iter()
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
