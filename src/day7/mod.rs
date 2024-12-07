fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let rest = parts.next().unwrap();
            let values = rest.split(' ').map(|v| v.parse().unwrap()).collect();
            (result, values)
        })
        .collect()
}

fn check(result: u64, numbers: &[u64], so_far: u64, i: usize) -> bool {
    if i == numbers.len() {
        return so_far == result;
    }

    check(result, numbers, so_far + numbers[i], i + 1)
        || check(result, numbers, so_far * numbers[i], i + 1)
}

fn check_part_2(result: u64, numbers: &[u64], so_far: u64, i: usize) -> bool {
    if i == numbers.len() {
        return so_far == result;
    }

    check_part_2(result, numbers, so_far + numbers[i], i + 1)
        || check_part_2(result, numbers, so_far * numbers[i], i + 1)
        || check_part_2(
            result,
            numbers,
            format!("{}{}", so_far, numbers[i]).parse().unwrap(),
            i + 1,
        )
}

pub fn part_1(input: &str) -> u64 {
    let input = parse_input(input);
    let mut sum = 0;

    for (result, numbers) in input {
        if check(result, &numbers, 0, 0) {
            sum += result;
        }
    }

    sum
}

pub fn part_2(input: &str) -> u64 {
    let input = parse_input(input);
    let mut sum = 0;

    for (result, numbers) in input {
        if check_part_2(result, &numbers, 0, 0) {
            sum += result;
        }
    }

    sum
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
    let data = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(part_1(data), 3749);
}

#[test]
fn test_part_2() {
    let data = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(part_2(data), 11387);
}
