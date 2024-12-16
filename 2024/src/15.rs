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

fn try_move(grid: &mut Grid<u8>, pos: Vec2, dir: Dir) -> bool {
    let next_pos = pos.neighbor(dir);
    match grid[next_pos] {
        b'#' => false,
        b'O' => {
            if try_move(grid, next_pos, dir) {
                grid.swap(pos, next_pos);
                true
            } else {
                false
            }
        }
        b'.' => {
            grid.swap(pos, next_pos);
            true
        }
        _ => unreachable!("Invalid character"),
    }
}

fn part_1(input: &str) -> i32 {
    let (mut grid, directions) = parse_input(input);
    let mut pos = grid.find(|&c| c == b'@').unwrap();

    for dir in directions {
        if try_move(&mut grid, pos, dir) {
            pos = pos.neighbor(dir);
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

fn can_move_p2(grid: &Grid<u8>, pos: Vec2, dir: Dir) -> bool {
    let next_pos = pos.neighbor(dir);
    match grid[next_pos] {
        b'#' => false,
        b'[' => {
            if can_move_p2(grid, next_pos, dir) {
                if let Dir::Up | Dir::Down = dir {
                    can_move_p2(grid, next_pos.neighbor(Dir::Right), dir)
                } else {
                    true
                }
            } else {
                false
            }
        }
        b']' => {
            if can_move_p2(grid, next_pos, dir) {
                if let Dir::Up | Dir::Down = dir {
                    can_move_p2(grid, next_pos.neighbor(Dir::Left), dir)
                } else {
                    true
                }
            } else {
                false
            }
        }
        b'.' => true,
        _ => unreachable!("Invalid character"),
    }
}

fn move_p2(grid: &mut Grid<u8>, pos: Vec2, dir: Dir) {
    let next_pos = pos.neighbor(dir);
    match grid[next_pos] {
        b'#' => (),
        b'[' => {
            move_p2(grid, next_pos, dir);
            if let Dir::Up | Dir::Down = dir {
                move_p2(grid, next_pos.neighbor(Dir::Right), dir);
            }
            grid.swap(pos, next_pos);
        }
        b']' => {
            move_p2(grid, next_pos, dir);
            if let Dir::Up | Dir::Down = dir {
                move_p2(grid, next_pos.neighbor(Dir::Left), dir);
            }
            grid.swap(pos, next_pos);
        }
        b'.' => {
            grid.swap(pos, next_pos);
        }
        _ => unreachable!("Invalid character"),
    }
}

fn part_2(input: &str) -> i32 {
    let (mut grid, directions) = parse_input(input);
    grid = grid.flat_map(|c| match c {
        b'#' => *b"##",
        b'O' => *b"[]",
        b'.' => *b"..",
        b'@' => *b"@.",
        _ => unreachable!("Invalid character"),
    });
    let mut pos = grid.find(|&c| c == b'@').unwrap();

    for dir in directions {
        if can_move_p2(&grid, pos, dir) {
            move_p2(&mut grid, pos, dir);
            pos = pos.neighbor(dir);
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
