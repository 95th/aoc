use std::collections::{HashMap, HashSet, VecDeque};

use aoc_util::{Dir, Grid};

fn main() {
    let input = include_str!("../input/16.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|&c| c == b'S').unwrap();

    let mut points = usize::MAX;
    let mut visited = grid.with_fill(usize::MAX);
    let mut pending = VecDeque::new();
    pending.push_back((start, Dir::Right, 0));

    while let Some((pos, dir, so_far)) = pending.pop_front() {
        if visited[pos] < so_far || grid[pos] == b'#' {
            continue;
        }
        if grid[pos] == b'E' {
            visited[pos] = visited[pos].min(so_far);
            points = points.min(so_far);
            continue;
        }
        visited[pos] = visited[pos].min(so_far);
        let mut add_pending = |dir: Dir, cost: usize| {
            if grid[pos.neighbor(dir)] != b'#' {
                pending.push_back((pos.neighbor(dir), dir, so_far + cost));
            }
        };
        add_pending(dir, 1);
        add_pending(dir.turn_left(), 1001);
        add_pending(dir.turn_right(), 1001);
    }

    points
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|&c| c == b'S').unwrap();

    let mut points = usize::MAX;
    let mut visited = grid.with_fill([usize::MAX; 4]);
    let mut pending = VecDeque::new();
    pending.push_back((start, Dir::Right, 0, Vec::new()));
    let mut path_map = HashMap::new();

    while let Some((pos, dir, so_far, mut path)) = pending.pop_front() {
        if visited[pos][dir as usize] < so_far || grid[pos] == b'#' {
            continue;
        }
        path.push(pos);
        if grid[pos] == b'E' {
            visited[pos][dir as usize] = visited[pos][dir as usize].min(so_far);
            points = points.min(so_far);
            path_map
                .entry(so_far)
                .or_insert_with(HashSet::new)
                .extend(path);
            continue;
        }
        visited[pos][dir as usize] = visited[pos][dir as usize].min(so_far);
        let mut add_pending = |dir: Dir, cost: usize| {
            if grid[pos.neighbor(dir)] != b'#' {
                pending.push_back((pos.neighbor(dir), dir, so_far + cost, path.clone()));
            }
        };
        add_pending(dir, 1);
        add_pending(dir.turn_left(), 1001);
        add_pending(dir.turn_right(), 1001);
    }

    path_map[&points].len()
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
