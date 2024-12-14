use std::fs::File;
use std::io::Write;

use crate::{dist::Dist, grid::Grid, pt::Pt};

fn parse_input(input: &str) -> Vec<(Pt, Dist)> {
    let regex = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse().unwrap();
            let y = cap[2].parse().unwrap();
            let vx = cap[3].parse().unwrap();
            let vy = cap[4].parse().unwrap();
            (Pt::new(x, y), Dist::new(vx, vy))
        })
        .collect()
}

pub fn part_1(input: &str, width: i32, height: i32) -> usize {
    let mut robots = parse_input(input);

    for _ in 0..100 {
        for (pos, velocity) in &mut robots {
            *pos += *velocity;
            pos.i = (pos.i + width) % width;
            pos.j = (pos.j + height) % height;
        }
    }

    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut quad = [0, 0, 0, 0];

    for (pos, _) in robots {
        if pos.i < mid_x && pos.j < mid_y {
            quad[0] += 1;
        } else if pos.i < mid_x && pos.j > mid_y {
            quad[1] += 1;
        } else if pos.i > mid_x && pos.j < mid_y {
            quad[2] += 1;
        } else if pos.i > mid_x && pos.j > mid_y {
            quad[3] += 1;
        }
    }

    quad.into_iter().product()
}

pub fn part_2(input: &str, width: i32, height: i32) {
    let mut robots = parse_input(input);
    let mut file = File::create("output.txt").unwrap();
    for i in 0..10000 {
        for (pos, velocity) in &mut robots {
            *pos += *velocity;
            pos.i = (pos.i + width) % width;
            pos.j = (pos.j + height) % height;
        }

        let mut grid = Grid::new(height as usize, width as usize, ' ');
        for (pos, _) in &robots {
            grid[*pos] = '#';
        }

        writeln!(&mut file, "Iteration: {}\n", i).unwrap();
        writeln!(&mut file, "{grid}").unwrap();
    }
}

#[test]
fn run_part_1() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_1(input, 101, 103));
}

#[test]
fn run_part_2() {
    let input = include_str!("input.txt");
    part_2(input, 101, 103);
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
