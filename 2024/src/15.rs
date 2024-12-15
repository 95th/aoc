use std::collections::HashSet;

use aoc_util::{Dir, Grid, Vec2};

fn main() {
    let input = include_str!("../input/15.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> (Grid<u8>, Vec<Dir>) {
    let (grid, directions) = input.split_once("\n\n").unwrap();
    let grid = Grid::from_bytes(grid);
    let directions = directions
        .lines()
        .flat_map(|line| line.bytes().map(Dir::from))
        .collect();
    (grid, directions)
}

fn part_1(input: &str) -> i32 {
    let (mut grid, directions) = parse_input(input);
    let mut pos = grid.find(|&c| c == b'@').unwrap();

    for dir in directions {
        let next_pos = pos.neighbor(dir);

        match grid[next_pos] {
            b'#' => {
                // No movement
            }
            b'O' => {
                let mut dot_pos = pos.neighbor(dir);
                while grid.get(dot_pos) == Some(&b'O') {
                    dot_pos = dot_pos.neighbor(dir);
                }
                if grid.get(dot_pos) == Some(&b'.') {
                    grid[pos] = b'.';
                    grid[dot_pos] = b'O';
                    grid[next_pos] = b'@';
                    pos = next_pos;
                }
            }
            b'.' => {
                grid[pos] = b'.';
                grid[next_pos] = b'@';
                pos = next_pos;
            }
            _ => unreachable!("Invalid character"),
        }
    }

    let mut sum = 0;
    for point in grid.points() {
        if grid[point] == b'O' {
            sum += 100 * point.y + point.x;
        }
    }

    sum
}

fn widen_grid(grid: Grid<u8>) -> Grid<u8> {
    let mut wide_grid = Grid::new(grid.width() * 2, grid.height(), b'.');
    for point in grid.points() {
        let new_point = Vec2::new(point.x * 2, point.y);
        match grid[point] {
            b'@' => {
                wide_grid[new_point] = b'@';
            }
            b'O' => {
                wide_grid[new_point] = b'[';
                wide_grid[new_point.neighbor(Dir::Right)] = b']';
            }
            b'#' => {
                wide_grid[new_point] = b'#';
                wide_grid[new_point.neighbor(Dir::Right)] = b'#';
            }
            b'.' => (),
            _ => unreachable!("Invalid character"),
        }
    }
    wide_grid
}

fn part_2(input: &str) -> i32 {
    let (mut grid, directions) = parse_input(input);
    grid = widen_grid(grid);
    let mut pos = grid.find(|&c| c == b'@').unwrap();

    for dir in directions {
        let next_pos = pos.neighbor(dir);

        match grid[next_pos] {
            b'#' => {
                // No movement
            }
            b'[' | b']' => match dir {
                Dir::Left | Dir::Right => {
                    let mut dot_pos = pos.neighbor(dir);
                    while let Some(b'[' | b']') = grid.get(dot_pos) {
                        dot_pos = dot_pos.neighbor(dir);
                    }
                    if grid.get(dot_pos) == Some(&b'.') {
                        let mut a = dot_pos;
                        let mut b = a.neighbor(dir.inverse());
                        while a != pos {
                            grid[a] = grid[b];
                            a = b;
                            b = b.neighbor(dir.inverse());
                        }
                        grid[pos] = b'.';
                        grid[next_pos] = b'@';
                        pos = next_pos;
                    }
                }
                Dir::Up | Dir::Down => {
                    let mut layers = vec![];
                    let mut current_layer = HashSet::new();
                    current_layer.insert(next_pos);
                    if grid[next_pos] == b'[' {
                        current_layer.insert(next_pos.neighbor(Dir::Right));
                    } else {
                        current_layer.insert(next_pos.neighbor(Dir::Left));
                    }

                    let found = loop {
                        if current_layer.iter().all(|&p| grid[p.neighbor(dir)] == b'.') {
                            layers.push(current_layer);
                            break true;
                        }

                        if current_layer.iter().any(|&p| grid[p.neighbor(dir)] == b'#') {
                            break false;
                        }

                        let mut next_layer = HashSet::new();
                        for &p in current_layer.iter() {
                            let p = p.neighbor(dir);
                            if grid[p] == b'[' {
                                next_layer.insert(p);
                                next_layer.insert(p.neighbor(Dir::Right));
                            } else if grid[p] == b']' {
                                next_layer.insert(p);
                                next_layer.insert(p.neighbor(Dir::Left));
                            }
                        }
                        layers.push(current_layer);
                        current_layer = next_layer;
                    };

                    if !found {
                        continue;
                    }

                    while let Some(layer) = layers.pop() {
                        for p in layer {
                            grid.swap(p, p.neighbor(dir));
                        }
                    }
                    grid[pos] = b'.';
                    grid[next_pos] = b'@';
                    pos = next_pos;
                }
            },
            b'.' => {
                grid[pos] = b'.';
                grid[next_pos] = b'@';
                pos = next_pos;
            }
            _ => unreachable!("Invalid character"),
        }
    }

    let mut sum = 0;
    for point in grid.points() {
        if grid[point] == b'[' {
            sum += 100 * point.y + point.x;
        }
    }

    sum
}

#[test]
fn test_part_1() {
    let data = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    assert_eq!(part_1(data), 2028);
}

#[test]
fn test_part_1_larger() {
    let data = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!(part_1(data), 10092);
}

#[test]
fn test_part_2() {
    let data = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    assert_eq!(part_2(data), 618);
}

#[test]
fn test_part_2_larger() {
    let data = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!(part_2(data), 9021);
}

#[test]
fn test_part_2_small() {
    let data = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!(part_2(data), 9021);
}
