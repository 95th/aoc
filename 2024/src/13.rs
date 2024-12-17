use aoc_util::Parse;

fn main() {
    let input = include_str!("../input/13.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug)]
struct Vec2f {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Machine {
    a: Vec2f,
    b: Vec2f,
    prize: Vec2f,
}

impl Machine {
    fn parse(input: &str) -> Vec<Self> {
        input.parse_regex(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
            |[ax, ay, bx, by, px, py]| {
                let button_a = Vec2f { x: ax, y: ay };
                let button_b = Vec2f { x: bx, y: by };
                let prize = Vec2f { x: px, y: py };
                Self {
                    a: button_a,
                    b: button_b,
                    prize,
                }
            },
        )
    }
}

fn find_min_tokens(machine: Machine) -> usize {
    let Machine { a, b, prize } = machine;
    let b_value = (prize.x * a.y - prize.y * a.x) / (b.x * a.y - b.y * a.x);
    let a_value = (prize.x - b_value * b.x) / a.x;
    if a_value < 0.0 || b_value < 0.0 || a_value.fract() != 0.0 || b_value.fract() != 0.0 {
        return 0;
    }
    a_value as usize * 3 + b_value as usize
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for machine in Machine::parse(input) {
        total += find_min_tokens(machine);
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut total = 0;
    for mut machine in Machine::parse(input) {
        machine.prize.x += 10000000000000.0;
        machine.prize.y += 10000000000000.0;
        total += find_min_tokens(machine);
    }
    total
}

#[test]
fn test_part_1() {
    let data = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    assert_eq!(part_1(data), 480);
}

#[test]
fn test_part_2() {
    let data = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    assert_eq!(part_2(data), 875318608908);
}
