fn main() {
    let input = include_str!("../input/17.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> (Vec<u128>, Vec<u8>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    let program = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    (registers, program)
}

fn evaluate(mut registers: [u128; 3], program: &[u8]) -> Vec<u8> {
    let mut ip = 0;
    let mut output = vec![];

    macro_rules! combo_value {
        () => {
            match program[ip + 1] {
                value @ 0..=3 => value as u128,
                value @ 4..=6 => registers[value as usize - 4],
                7 => unreachable!("Reserved"),
                _ => unreachable!("Invalid value"),
            }
        };
    }

    while ip < program.len() {
        match program[ip] {
            0 => {
                let combo_value = combo_value!();
                registers[0] /= 2_u128.pow(combo_value as u32);
                ip += 2;
            }
            1 => {
                registers[1] ^= program[ip + 1] as u128;
                ip += 2;
            }
            2 => {
                let combo_value = combo_value!();
                registers[1] = combo_value % 8;
                ip += 2;
            }
            3 => {
                if registers[0] == 0 {
                    ip += 2;
                } else {
                    ip = program[ip + 1] as usize;
                }
            }
            4 => {
                registers[1] ^= registers[2];
                ip += 2;
            }
            5 => {
                let combo_value = combo_value!();
                output.push((combo_value % 8) as u8);
                ip += 2;
            }
            6 => {
                let combo_value = combo_value!();
                registers[1] = registers[0] / 2_u128.pow(combo_value as u32);
                ip += 2;
            }
            7 => {
                let combo_value = combo_value!();
                registers[2] = registers[0] / 2_u128.pow(combo_value as u32);
                ip += 2;
            }
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
    let output = evaluate([a, b, c], &program);
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

    for matched in 0..program.len() {
        a *= 8;
        while evaluate([a, b, c], &program) != program[program.len() - (matched + 1)..] {
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
