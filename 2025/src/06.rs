use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/06.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i64 {
    let (input, operations) = input.rsplit_once("\n").unwrap();
    let grid = Grid::<i64>::parse(input);
    let operations = operations.split_whitespace().collect::<Vec<_>>();

    let mut output = 0;

    for (i, col) in grid.cols().enumerate() {
        if operations[i] == "+" {
            output += col.sum::<i64>();
        } else {
            output += col.product::<i64>();
        }
    }
    output
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::from_bytes(input);

    let mut output = 0;
    let mut operator = b'+';
    let mut values = Vec::<String>::new();
    for col in grid.cols() {
        let mut number = String::new();
        for &v in col {
            match v {
                b'+' | b'*' => operator = v,
                v => number.push(v as char),
            }
        }

        if number.trim().is_empty() {
            if operator == b'+' {
                output += values
                    .iter()
                    .map(|v| v.trim().parse::<i64>().unwrap())
                    .sum::<i64>();
            } else {
                output += values
                    .iter()
                    .map(|v| v.trim().parse::<i64>().unwrap())
                    .product::<i64>();
            }
            values.clear();
        } else {
            values.push(number);
        }
    }

    if !values.is_empty() {
        if operator == b'+' {
            output += values
                .iter()
                .map(|v| v.trim().parse::<i64>().unwrap())
                .sum::<i64>();
        } else {
            output += values
                .iter()
                .map(|v| v.trim().parse::<i64>().unwrap())
                .product::<i64>();
        }
    }

    output
}

#[test]
fn test_part_1() {
    let data = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    assert_eq!(part_1(data), 4277556);
}

#[test]
fn test_part_2() {
    let data = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    assert_eq!(part_2(data), 3263827);
}
