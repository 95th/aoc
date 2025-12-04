use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/04.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_bytes(input);
    let mut count = 0;
    for p in grid.points() {
        if grid[p] == b'@' && grid.neighbors_all(p).filter(|n| grid[*n] == b'@').count() < 4 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let mut grid = Grid::from_bytes(input);
    let mut count = 0;
    loop {
        let mut changed = false;
        let mut new_grid = grid.clone();
        for p in grid.points() {
            if grid[p] == b'@' && grid.neighbors_all(p).filter(|n| grid[*n] == b'@').count() < 4 {
                new_grid[p] = b'.';
                count += 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
        grid = new_grid;
    }
    count
}

#[test]
fn test_part_1() {
    let data = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(part_1(data), 13);
}

#[test]
fn test_part_2() {
    let data = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(part_2(data), 43);
}
