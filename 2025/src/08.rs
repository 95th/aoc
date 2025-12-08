use std::collections::HashMap;

use aoc_util::{UnionFind, Vec3};

fn main() {
    let input = include_str!("../input/08.txt");
    println!("Part 1: {}", part_1(input, 1000));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str, circuits: usize) -> usize {
    let junction_boxes: Vec<_> = input.lines().map(|line| Vec3::parse_csv(line)).collect();

    let mut pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            if i != j {
                let dist = junction_boxes[i].euclidean_dist(junction_boxes[j]);
                pairs.push((dist, (i, j)));
            }
        }
    }
    pairs.sort_by_key(|(dist, _)| *dist);

    let mut connections = UnionFind::new(junction_boxes.len());
    for (_, (i, j)) in pairs.into_iter().take(circuits) {
        connections.union(i, j);
    }

    let mut sizes = HashMap::new();
    for i in 0..junction_boxes.len() {
        *sizes.entry(connections.find(i)).or_insert(0) += 1;
    }
    let mut sizes = sizes.into_values().collect::<Vec<_>>();
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn part_2(input: &str) -> isize {
    let junction_boxes: Vec<Vec3> = input.lines().map(|line| Vec3::parse_csv(line)).collect();

    let mut pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            if i != j {
                let dist = junction_boxes[i].euclidean_dist(junction_boxes[j]);
                pairs.push((dist, (i, j)));
            }
        }
    }
    pairs.sort_by_key(|(dist, _)| *dist);

    let mut connections = UnionFind::new(junction_boxes.len());
    let mut pair = (0, 0);
    for (_, (i, j)) in pairs.into_iter() {
        if connections.union(i, j) {
            pair = (i, j);
        }
    }

    junction_boxes[pair.0].x * junction_boxes[pair.1].x
}

#[test]
fn test_part_1() {
    let data = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(part_1(data, 10), 40);
}

#[test]
fn test_part_2() {
    let data = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(part_2(data), 25272);
}
