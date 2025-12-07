use aoc_util::{Dir, Grid};

fn main() {
    let input = include_str!("../input/07.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|c| *c == b'S');
    let mut splits = 0;

    let mut pending = Vec::new();
    pending.push(start);

    let mut visited = grid.with_fill(false);
    while let Some(pos) = pending.pop() {
        if visited[pos] {
            continue;
        }
        visited[pos] = true;
        let next = pos.neighbor(Dir::Down);
        if let Some(c) = grid.get(next) {
            match c {
                b'.' => pending.push(next),
                b'^' => {
                    splits += 1;
                    pending.push(next.neighbor(Dir::Left));
                    pending.push(next.neighbor(Dir::Right));
                }
                _ => unreachable!("Invalid character"),
            }
        }
    }

    splits
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = grid.find(|c| *c == b'S');

    let mut timelines = grid.with_fill(0);
    timelines[start] = 1;

    for pos in grid.points() {
        let next = pos.neighbor(Dir::Down);
        if let Some(&c) = grid.get(next) {
            match c {
                b'.' | b'S' => {
                    timelines[next] += timelines[pos];
                }
                b'^' => {
                    timelines[next.neighbor(Dir::Left)] += timelines[pos];
                    timelines[next.neighbor(Dir::Right)] += timelines[pos];
                }
                c => unreachable!("Invalid character: {}", c as char),
            }
        }
    }
    timelines.rows().last().unwrap().iter().sum::<usize>()
}

#[test]
fn test_part_1() {
    let data = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(part_1(data), 21);
}

#[test]
fn test_part_2() {
    let data = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(part_2(data), 40);
}

#[test]
fn test_part_2_small() {
    let data = r".......S.......
...............
.......^.......
...............
......^.^......
...............";
    assert_eq!(part_2(data), 4);
}
