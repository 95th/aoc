use aoc_util::Parse;

fn main() {
    let input = include_str!("../input/17.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> (Vec<u128>, Vec<u8>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers.lines().map(|s| s.parse_after_colon()).collect();
    let program = program.after_colon().list(",");
    (registers, program)
}

fn evaluate(mut a: u128, mut b: u128, mut c: u128, program: &[u8]) -> Vec<u8> {
    let mut ip = 0;
    let mut output = vec![];

    macro_rules! combo {
        ($operand:expr) => {
            match $operand {
                0..=3 => $operand as u128,
                4..=6 => [a, b, c][$operand as usize - 4],
                7 => unreachable!("Reserved"),
                _ => unreachable!("Invalid value"),
            }
        };
    }

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        ip += 2;
        match opcode {
            0 => a /= 2_u128.pow(combo!(operand) as u32),
            1 => b ^= operand as u128,
            2 => b = combo!(operand) % 8,
            3 => {
                if a != 0 {
                    ip = operand as usize;
                }
            }
            4 => b ^= c,
            5 => output.push((combo!(operand) % 8) as u8),
            6 => b = a / 2_u128.pow(combo!(operand) as u32),
            7 => c = a / 2_u128.pow(combo!(operand) as u32),
            _ => unreachable!("Invalid instruction"),
        }
    }
    output
}

fn part_1(input: &str) -> String {
    let (registers, program) = parse_input(input);
    let a = registers[0];
    let b = registers[1];
    let c = registers[2];
    let output = evaluate(a, b, c, &program);
    output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_2(input: &str) -> u128 {
    let (registers, program) = parse_input(input);

    let mut a = 0;
    let b = registers[1];
    let c = registers[2];

    for i in 0..program.len() {
        a *= 8;
        while evaluate(a, b, c, &program) != program[program.len() - (i + 1)..] {
            a += 1;
        }
    }
    a
}

#[test]
fn test_part_1() {
    let data = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    assert_eq!(part_1(data), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn test_part_1_small() {
    let data = r"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
    assert_eq!(part_1(data), "0,1,2");
}

#[test]
fn test_part_1_small_2() {
    let data = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    assert_eq!(part_1(data), "4,2,5,6,7,7,7,7,3,1,0");
}

#[test]
fn test_part_1_small_3() {
    let data = r"Register A: 0
Register B: 29
Register C: 0

Program: 1,7";
    assert_eq!(part_1(data), "");
}

#[test]
fn test_part_1_small_4() {
    let data = r"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";
    assert_eq!(part_1(data), "");
}

#[test]
fn test_part_2() {
    let data = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    assert_eq!(part_2(data), 117440);
}
