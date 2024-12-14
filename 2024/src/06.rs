use aoc_util::{Dir, Grid, Vec2};

fn main() {
    let input = include_str!("../input/06.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn direction(guard: u8) -> Dir {
    match guard {
        b'^' => Dir::Up,
        b'v' => Dir::Down,
        b'<' => Dir::Left,
        b'>' => Dir::Right,
        _ => unreachable!("what"),
    }
}

fn guard_position(grid: &Grid<u8>) -> Vec2 {
    grid.find(|c| matches!(c, b'^' | b'v' | b'<' | b'>'))
        .unwrap()
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let start = guard_position(&grid);
    let mut dir = direction(grid[start]);

    let mut current = start;
    let mut steps = grid.map(false);
    let mut count = 0;

    while grid.contains_point(current) {
        if !steps[current] {
            steps[current] = true;
            count += 1;
        }
        if grid.get(current.step(dir)) == Some(&b'#') {
            dir = dir.turn_right();
        }
        current = current.step(dir);
    }

    count
}

fn is_loop(grid: &Grid<u8>, start: Vec2) -> bool {
    let mut dir = direction(grid[start]);
    let mut steps = grid.map([false; 4]);

    let mut current = start;
    while grid.contains_point(current) {
        if steps[current][dir as usize] {
            return true;
        }
        steps[current][dir as usize] = true;

        let next = current.step(dir);
        if grid.get(next) == Some(&b'#') {
            dir = dir.turn_right();
        } else {
            current = next;
        }
    }

    false
}

fn part_2(input: &str) -> usize {
    let mut grid = Grid::from_bytes(input);
    let start = guard_position(&grid);

    let mut count = 0;
    for point in grid.points() {
        if point == start || grid[point] == b'#' {
            continue;
        }
        grid[point] = b'#';
        if is_loop(&grid, start) {
            count += 1;
        }
        grid[point] = b'.';
    }
    count
}

#[test]
fn test_part_1() {
    let data = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part_1(data), 41);
}

#[test]
fn test_part_2() {
    let data = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part_2(data), 6);
}
