use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

use aoc_util::{Dir, Grid, Vec2};

fn main() {
    let input = include_str!("../input/16.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|&c| c == b'S').unwrap();

    let mut points = usize::MAX;
    let mut visited = grid.fill(usize::MAX);
    let mut pending = vec![(start, Dir::Right, 0)];

    while let Some((pos, dir, so_far)) = pending.pop() {
        if visited[pos] < so_far || grid[pos] == b'#' {
            continue;
        }
        if grid[pos] == b'E' {
            visited[pos] = visited[pos].min(so_far);
            points = points.min(so_far);
            continue;
        }
        visited[pos] = visited[pos].min(so_far);
        pending.push((pos.neighbor(dir), dir, so_far + 1));
        pending.push((
            pos.neighbor(dir.turn_left()),
            dir.turn_left(),
            so_far + 1001,
        ));
        pending.push((
            pos.neighbor(dir.turn_right()),
            dir.turn_right(),
            so_far + 1001,
        ));
    }

    points
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|&c| c == b'S').unwrap();

    let mut points = usize::MAX;
    let mut visited = grid.fill([usize::MAX; 4]);
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
        pending.push_back((pos.neighbor(dir), dir, so_far + 1, path.clone()));
        pending.push_back((
            pos.neighbor(dir.turn_left()),
            dir.turn_left(),
            so_far + 1001,
            path.clone(),
        ));
        pending.push_back((
            pos.neighbor(dir.turn_right()),
            dir.turn_right(),
            so_far + 1001,
            path.clone(),
        ));
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
