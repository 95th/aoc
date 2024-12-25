use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/25.txt");
    println!("Part 1: {}", part_1(input));
}

fn pins(grid: Grid<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    for col in grid.cols() {
        out.push(col.filter(|&&s| s == b'#').count() as u8 - 1)
    }
    out
}

fn part_1(input: &str) -> usize {
    let items: Vec<_> = input.split("\n\n").map(|s| Grid::from_bytes(s)).collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for grid in items {
        if grid.rows().next().unwrap().iter().all(|&x| x == b'#')
            && grid.rows().last().unwrap().iter().all(|&x| x == b'.')
        {
            locks.push(pins(grid));
        } else {
            keys.push(pins(grid));
        }
    }

    let mut matching = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5) {
                matching += 1;
            }
        }
    }
    matching
}

#[test]
fn test_part_1() {
    let data = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    assert_eq!(part_1(data), 3);
}
