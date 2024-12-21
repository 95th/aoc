use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_util::{Dir, Grid};

#[derive(PartialEq, Eq)]
struct Sort<T>(usize, T);

impl<T: Eq> PartialOrd for Sort<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Sort<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() {
    let input = include_str!("../input/16.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|&c| c == b'S');
    let mut visited = grid.with_fill([false; 4]);

    let mut min_dist = usize::MAX;
    let mut queue = BinaryHeap::new();
    queue.push(Sort(0, (start, Dir::Right)));

    while let Some(Sort(dist, (pos, dir))) = queue.pop() {
        if grid[pos] == b'E' {
            min_dist = min_dist.min(dist);
            continue;
        }
        if visited[pos][dir as usize] {
            continue;
        }
        visited[pos][dir as usize] = true;
        let next = pos.neighbor(dir);
        if grid[next] != b'#' {
            queue.push(Sort(dist + 1, (next, dir)));
        }
        queue.push(Sort(dist + 1000, (pos, dir.turn_left())));
        queue.push(Sort(dist + 1000, (pos, dir.turn_right())));
    }

    min_dist
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|&c| c == b'S');

    let mut min_dist = usize::MAX;
    let mut visited = grid.with_fill([usize::MAX; 4]);
    let mut path_map = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(Sort(0, (start, Dir::Right, vec![start])));

    while let Some(Sort(dist, (pos, dir, path))) = queue.pop() {
        if visited[pos][dir as usize] < dist {
            continue;
        }
        visited[pos][dir as usize] = dist;
        if grid[pos] == b'E' {
            min_dist = min_dist.min(dist);
            path_map
                .entry(dist)
                .or_insert_with(HashSet::new)
                .extend(path);
            continue;
        }

        let next = pos.neighbor(dir);
        if grid[next] != b'#' {
            let mut path = path.clone();
            path.push(next);
            queue.push(Sort(dist + 1, (next, dir, path)));
        }
        queue.push(Sort(dist + 1000, (pos, dir.turn_left(), path.clone())));
        queue.push(Sort(dist + 1000, (pos, dir.turn_right(), path)));
    }

    path_map[&min_dist].len()
}

#[test]
fn test_part_1_small() {
    let data = r"#####
#..E#
#S..#
#####";
    assert_eq!(part_1(data), 1003);
}

#[test]
fn test_part_1_small_2() {
    let data = r"#####
#..E#
#S.##
#####";
    assert_eq!(part_1(data), 2003);
}

#[test]
fn test_part_1_small_3() {
    let data = r"#######
#...#E#
#S#...#
#######";
    assert_eq!(part_1(data), 5007);
}

#[test]
fn test_part_1() {
    let data = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part_1(data), 7036);
}

#[test]
fn test_part_1_larger() {
    let data = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    assert_eq!(part_1(data), 11048);
}

#[test]
fn test_part_2_small() {
    let data = r"#####
#..E#
#S..#
#####";
    assert_eq!(part_2(data), 4);
}

#[test]
fn test_part_2_small_2() {
    let data = r"#####
###E#
#...#
#.#.#
#...#
#S###
#####";
    assert_eq!(part_2(data), 10);
}

#[test]
fn test_part_2_small_3() {
    let data = r"########
#.....E#
###.#.##
#...#..#
#.#.#.##
#.....##
#.###.##
#S..#..#
########";
    assert_eq!(part_2(data), 20);
}

#[test]
fn test_part_2() {
    let data = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part_2(data), 45);
}

#[test]
fn test_part_2_larger() {
    let data = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    assert_eq!(part_2(data), 64);
}
