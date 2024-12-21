use std::{collections::VecDeque, usize};

use aoc_util::{Grid, Vec2};

fn main() {
    let input = include_str!("../input/20.txt");
    println!("Part 1: {}", part_1(input, 100));
    println!("Part 2: {}", part_2(input, 100));
}

fn distances(grid: &Grid<u8>, start: Vec2) -> (Grid<usize>, Vec<Vec2>) {
    let mut visited = grid.with_fill(false);
    let mut pending = VecDeque::new();
    pending.push_back((start, 0));

    let mut distances = grid.with_fill(usize::MAX);
    let mut path = Vec::new();

    while let Some((pos, so_far)) = pending.pop_front() {
        visited[pos] = true;
        path.push(pos);
        distances[pos] = so_far;
        for next in grid.neighbors(pos) {
            if grid[next] != b'#' && !visited[next] {
                pending.push_back((next, so_far + 1));
            }
        }
    }
    (distances, path)
}

fn solve(input: &str, min_save: usize, allowed_skips: usize) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|c| *c == b'S');
    let end = grid.find(|c| *c == b'E');

    let (dist, path) = distances(&grid, start);
    let (rev_dist, _) = distances(&grid, end);

    let mut count = 0;
    for p in path {
        for q in grid.points() {
            if rev_dist[q] == usize::MAX {
                continue;
            }

            let distance = p.manhattan_dist(q) as usize;
            if distance > allowed_skips {
                continue;
            }

            if dist[p] + distance + rev_dist[q] <= dist[end] - min_save {
                count += 1;
            }
        }
    }

    count
}

fn part_1(input: &str, min_save: usize) -> usize {
    solve(input, min_save, 2)
}

fn part_2(input: &str, min_save: usize) -> usize {
    solve(input, min_save, 20)
}

#[test]
fn test_part_1() {
    let data = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    assert_eq!(part_1(data, 20), 5);
}

#[test]
fn test_part_2() {
    let data = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    assert_eq!(part_2(data, 50), 285);
}
