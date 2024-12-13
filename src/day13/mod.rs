#[derive(Debug)]
struct Pt {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Machine {
    a: Pt,
    b: Pt,
    prize: Pt,
}

fn parse_button(line: &str) -> Pt {
    let regex = regex::Regex::new(r"X([+-]\d+), Y([+-]\d+)").unwrap();
    let caps = regex.captures(line).unwrap();
    Pt {
        x: caps[1].parse().unwrap(),
        y: caps[2].parse().unwrap(),
    }
}

fn parse_prize(line: &str) -> Pt {
    let regex = regex::Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
    let caps = regex.captures(line).unwrap();
    Pt {
        x: caps[1].parse().unwrap(),
        y: caps[2].parse().unwrap(),
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.split("\n\n").map(|group| {
        let mut lines = group.lines();
        let button_a = parse_button(lines.next().unwrap());
        let button_b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());
        Machine {
            a: button_a,
            b: button_b,
            prize,
        }
    })
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

pub fn part_1(input: &str) -> usize {
    let mut total = 0;
    for machine in parse_input(input) {
        total += find_min_tokens(machine);
    }
    total
}

pub fn part_2(input: &str) -> usize {
    let mut total = 0;
    for mut machine in parse_input(input) {
        machine.prize.x += 10000000000000.0;
        machine.prize.y += 10000000000000.0;
        total += find_min_tokens(machine);
    }
    total
}

#[test]
fn run_part_1() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_1(input));
}

#[test]
fn run_part_2() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_2(input));
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
