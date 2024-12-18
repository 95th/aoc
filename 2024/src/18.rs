use std::{collections::VecDeque, i32};

use aoc_util::{vec2, Grid, Parse, Vec2};

fn main() {
    let input = include_str!("../input/18.txt");
    println!("Part 1: {}", part_1(input, 71, 71, 1024));
    println!("Part 2: {}", part_2(input, 71, 71));
}

fn parse_input(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|s| {
            let (a, b) = s.pair(",");
            vec2(a, b)
        })
        .collect()
}

fn calculate_shortest_path(grid: &Grid<char>, start: Vec2, end: Vec2) -> i32 {
    let mut visited = grid.with_fill(false);
    let mut pending = VecDeque::new();
    pending.push_back((start, 0));

    while let Some((pos, so_far)) = pending.pop_front() {
        if pos == end {
            return so_far;
        }
        if visited[pos] {
            continue;
        }
        visited[pos] = true;
        for next in grid.neighbors(pos) {
            if grid[next] != '#' && !visited[next] {
                pending.push_back((next, so_far + 1));
            }
        }
    }
    -1
}

fn part_1(input: &str, width: usize, height: usize, bytes: usize) -> i32 {
    let points = parse_input(input);
    let mut grid = Grid::new(width, height, '.');

    for &p in points.iter().take(bytes) {
        grid[p] = '#';
    }

    let start = vec2(0, 0);
    let end = vec2(width as i32 - 1, height as i32 - 1);
    calculate_shortest_path(&grid, start, end)
}

fn part_2(input: &str, width: usize, height: usize) -> String {
    let points = parse_input(input);
    let mut grid = Grid::new(width, height, '.');

    let start = vec2(0, 0);
    let end = vec2(width as i32 - 1, height as i32 - 1);

    for p in points {
        grid[p] = '#';
        if calculate_shortest_path(&grid, start, end) == -1 {
            return format!("{},{}", p.x, p.y);
        }
    }

    String::new()
}

#[test]
fn test_part_1() {
    let data = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(part_1(data, 7, 7, 12), 22);
}

#[test]
fn test_part_2() {
    let data = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(part_2(data, 7, 7), "6,1");
}
