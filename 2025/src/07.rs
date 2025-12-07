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
    let mut has_beam = grid.with_fill(false);
    has_beam[start] = true;

    for pos in grid.points() {
        if !has_beam[pos] {
            continue;
        }
        let next = pos.neighbor(Dir::Down);
        match grid.get(next) {
            Some(b'.') => has_beam[next] = true,
            Some(b'^') => {
                has_beam[next.neighbor(Dir::Left)] = true;
                has_beam[next.neighbor(Dir::Right)] = true;
                splits += 1;
            }
            _ => (),
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
        match grid.get(next) {
            Some(b'.' | b'S') => timelines[next] += timelines[pos],
            Some(b'^') => {
                timelines[next.neighbor(Dir::Left)] += timelines[pos];
                timelines[next.neighbor(Dir::Right)] += timelines[pos];
            }
            _ => (),
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
