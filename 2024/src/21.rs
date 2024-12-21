use std::collections::HashMap;

use aoc_util::{Dir, Grid, ZERO};
use itertools::Itertools;

fn main() {
    let input = include_str!("../input/21.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn get_complexity_rec(
    id: usize,
    grids: &[&Grid<u8>],
    a: u8,
    b: u8,
    memo: &mut HashMap<(usize, u8, u8), usize>,
) -> usize {
    if let Some(&complexity) = memo.get(&(id, a, b)) {
        return complexity;
    }

    let g = &grids[id];
    let start = g.find(|c| *c == a);
    let end = g.find(|c| *c == b);
    let dist = end - start;

    if id == grids.len() - 1 {
        return dist.manhattan() as usize + 1;
    }

    if dist == ZERO {
        return 1;
    }

    let x = if dist.x >= 0 { b'>' } else { b'<' };
    let y = if dist.y >= 0 { b'v' } else { b'^' };

    let mut path = Vec::new();
    path.extend((0..dist.x.abs()).map(|_| x));
    path.extend((0..dist.y.abs()).map(|_| y));

    let mut path_sizes = Vec::new();
    'outer: for p in path.iter().permutations(path.len()) {
        let mut pos = start;
        let mut steps = 0;
        let mut a = b'A';
        for &&d in p.iter() {
            steps += get_complexity_rec(id + 1, grids, a, d, memo);
            pos = pos.neighbor(Dir::from(d));
            if g[pos] == b' ' {
                continue 'outer;
            }
            a = d;
        }
        steps += get_complexity_rec(id + 1, grids, **p.last().unwrap(), b'A', memo);
        path_sizes.push(steps);
    }

    let complexity = path_sizes.into_iter().min().unwrap();
    memo.insert((id, a, b), complexity);
    complexity
}

fn get_complexity(code: &str, robots: usize) -> usize {
    let num_grid = Grid::from_bytes("789\n456\n123\n 0A\n");
    let dir_grid = Grid::from_bytes(" ^A\n<v>");
    let mut memo = HashMap::new();

    let mut grids = vec![&dir_grid; robots];
    grids[0] = &num_grid;

    let mut complexity = 0;
    let mut a = b'A';
    for b in code.bytes() {
        complexity += get_complexity_rec(0, &grids, a, b, &mut memo);
        a = b;
    }
    complexity
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for code in input.lines() {
        let complexity = get_complexity(code, 3);
        let num_value: usize = code.replace("A", "").parse().unwrap();
        total += complexity * num_value;
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut total = 0;
    for code in input.lines() {
        let complexity = get_complexity(code, 26);
        let num_value: usize = code.replace("A", "").parse().unwrap();
        total += complexity * num_value;
    }
    total
}

#[test]
fn test_part_1() {
    let data = r"029A
980A
179A
456A
379A";
    assert_eq!(part_1(data), 126384);
}

#[test]
fn test_part_2() {
    let data = r"029A
980A
179A
456A
379A";
    assert_eq!(part_2(data), 154115708116294);
}
