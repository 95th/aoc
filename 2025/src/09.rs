use std::collections::VecDeque;

use aoc_util::{Grid, Parse, Vec2, vec2};

fn main() {
    let input = include_str!("../input/09.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.pair(",");
            vec2(x, y)
        })
        .collect()
}

fn area(a: Vec2, b: Vec2) -> isize {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn part_1(input: &str) -> isize {
    let points = parse(input);
    let mut max = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            max = max.max(area(points[i], points[j]));
        }
    }
    max
}

fn part_2(input: &str) -> isize {
    let points = parse(input);

    let mut xs: Vec<isize> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<isize> = points.iter().map(|p| p.y).collect();
    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();

    // Insert gap around every point and compress the point space
    let points_with_gap: Vec<Vec2> = points
        .iter()
        .map(|p| {
            vec2(
                2 * xs.binary_search(&p.x).unwrap() as isize + 1,
                2 * ys.binary_search(&p.y).unwrap() as isize + 1,
            )
        })
        .collect();

    let mut grid = Grid::new(2 * xs.len() + 1, 2 * ys.len() + 1, false);

    for i in 0..points_with_gap.len() {
        let a = points_with_gap[i];
        let b = points_with_gap[(i + 1) % points_with_gap.len()];
        let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
        let (y1, y2) = (a.y.min(b.y), a.y.max(b.y));
        for y in y1..=y2 {
            for x in x1..=x2 {
                grid[vec2(x, y)] = true;
            }
        }
    }

    // Flood fill exterior from (0,0)
    // Guaranteed to be outside because of gaps.
    let mut queue = VecDeque::from([vec2(0, 0)]);
    let mut exterior = grid.with_fill(false);
    exterior[vec2(0, 0)] = true;
    while let Some(p) = queue.pop_front() {
        for n in grid.neighbors(p) {
            if !grid[n] && !exterior[n] {
                exterior[n] = true;
                queue.push_back(n);
            }
        }
    }

    let mut max = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let rect_area = area(points[i], points[j]);
            if rect_area <= max {
                continue;
            }
            let (a, b) = (points_with_gap[i], points_with_gap[j]);
            let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
            let (y1, y2) = (a.y.min(b.y), a.y.max(b.y));
            let valid = (y1..=y2).all(|y| (x1..=x2).all(|x| !exterior[vec2(x, y)]));
            if valid {
                max = rect_area;
            }
        }
    }
    max
}

#[test]
fn test_part_1() {
    let data = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    assert_eq!(part_1(data), 50);
}

#[test]
fn test_part_2() {
    let data = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    assert_eq!(part_2(data), 24);
}
