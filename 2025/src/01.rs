fn main() {
    let input = include_str!("../input/01.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.replace("L", "-").replace("R", "+"))
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut password = 0;
    let mut current = 50;
    for line in lines {
        current = (current + line) % 100;
        if current == 0 {
            password += 1;
        }
    }
    password
}

fn part_2(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.replace("L", "-").replace("R", "+"))
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut password = 0;
    let mut current = 50;
    for line in lines {
        if line >= 0 {
            for _ in 0..line {
                current = (current + 1) % 100;
                if current == 0 {
                    password += 1;
                }
            }
        } else {
            for _ in 0..line.abs() {
                current = (current - 1) % 100;
                if current == 0 {
                    password += 1;
                }
            }
        }
    }
    password
}

#[test]
fn test_part_1() {
    let data = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part_1(data), 3);
}

#[test]
fn test_part_2() {
    let data = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part_2(data), 6);
}
