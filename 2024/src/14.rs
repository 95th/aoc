use std::io::Write;
use std::{fs::File, io::BufWriter};

use aoc_util::{vec2, Grid, Vec2};

fn main() {
    let input = include_str!("../input/14.txt");
    println!("Part 1: {}", part_1(input, 101, 103));
    part_2(input, 101, 103);
}

fn parse_input(input: &str) -> Vec<(Vec2, Vec2)> {
    let regex = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse().unwrap();
            let y = cap[2].parse().unwrap();
            let vx = cap[3].parse().unwrap();
            let vy = cap[4].parse().unwrap();
            (vec2(x, y), vec2(vx, vy))
        })
        .collect()
}

fn part_1(input: &str, width: i32, height: i32) -> usize {
    let mut robots = parse_input(input);

    for _ in 0..100 {
        for (pos, velocity) in &mut robots {
            pos.x = (pos.x + velocity.x + width) % width;
            pos.y = (pos.y + velocity.y + height) % height;
        }
    }

    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut quad = [0, 0, 0, 0];

    for (pos, _) in robots {
        if pos.x < mid_x && pos.y < mid_y {
            quad[0] += 1;
        } else if pos.x < mid_x && pos.y > mid_y {
            quad[1] += 1;
        } else if pos.x > mid_x && pos.y < mid_y {
            quad[2] += 1;
        } else if pos.x > mid_x && pos.y > mid_y {
            quad[3] += 1;
        }
    }

    quad.into_iter().product()
}

fn part_2(input: &str, width: i32, height: i32) {
    let mut robots = parse_input(input);
    let mut file = BufWriter::new(File::create("output.txt").unwrap());
    for i in 0..10000 {
        let mut grid = Grid::new(width as usize, height as usize, ' ');
        for (pos, velocity) in &mut robots {
            pos.x = (pos.x + velocity.x + width) % width;
            pos.y = (pos.y + velocity.y + height) % height;
            grid[*pos] = '#';
        }

        writeln!(&mut file, "Iteration: {}\n", i + 1).unwrap();
        writeln!(&mut file, "{grid}").unwrap();
    }
}

#[test]
fn test_part_1() {
    let data = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    assert_eq!(part_1(data, 11, 7), 12);
}
