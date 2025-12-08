use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/08.txt");
    println!("Part 1: {}", part_1(input, 1000));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

fn part_1(input: &str, circuits: usize) -> usize {
    let junction_boxes: Vec<Point> = input
        .lines()
        .map(|line| {
            let a: Vec<_> = line.split(',').map(|x| x.parse().unwrap()).collect();
            Point {
                x: a[0],
                y: a[1],
                z: a[2],
            }
        })
        .collect();

    let mut pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            if i != j {
                let dist = distance(&junction_boxes[i], &junction_boxes[j]);
                pairs.push((dist, (i, j)));
            }
        }
    }
    pairs.sort_by_key(|x| x.0);

    let mut connections = vec![0; junction_boxes.len()];
    for i in 0..junction_boxes.len() {
        connections[i] = i;
    }
    for (_, (i, j)) in pairs.into_iter().take(circuits) {
        union(&mut connections, i, j);
    }

    let mut sizes = HashMap::new();
    for i in 0..junction_boxes.len() {
        *sizes.entry(find(&connections, i)).or_insert(0) += 1;
    }
    let mut sizes = sizes.into_values().collect::<Vec<_>>();
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

fn part_2(input: &str) -> isize {
    let junction_boxes: Vec<Point> = input
        .lines()
        .map(|line| {
            let point: Vec<_> = line.split(',').map(|x| x.parse().unwrap()).collect();
            Point {
                x: point[0],
                y: point[1],
                z: point[2],
            }
        })
        .collect();

    let mut pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            if i != j {
                let dist = distance(&junction_boxes[i], &junction_boxes[j]);
                pairs.push((dist, (i, j)));
            }
        }
    }
    pairs.sort_by_key(|x| x.0);

    let mut connections = vec![0; junction_boxes.len()];
    for i in 0..junction_boxes.len() {
        connections[i] = i;
    }
    let mut pair = (0, 0);
    for (_, (i, j)) in pairs.into_iter() {
        if union(&mut connections, i, j) {
            pair = (i, j);
        }
    }

    junction_boxes[pair.0].x * junction_boxes[pair.1].x
}

fn union(connections: &mut [usize], a: usize, b: usize) -> bool {
    let root_a = find(connections, a);
    let root_b = find(connections, b);
    if root_a == root_b {
        return false;
    }
    connections[root_a] = root_b;
    for i in 0..connections.len() {
        if connections[i] == root_a {
            connections[i] = root_b;
        }
    }
    true
}

fn find(connections: &[usize], a: usize) -> usize {
    if connections[a] == a {
        a
    } else {
        find(connections, connections[a])
    }
}

fn distance(a: &Point, b: &Point) -> isize {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
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
