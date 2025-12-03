use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/03.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u64 {
    let g = Grid::from_bytes(input);
    let mut sum: u64 = 0;
    for row in g.rows() {
        let mut a = 0;
        let mut b = 0;
        for cell in row {
            let j = (*cell - b'0') as u64;
            if a == 0 {
                a = j;
            } else if b == 0 {
                b = j;
            } else if a * 10 + b < b * 10 + j {
                a = b;
                b = j;
            } else if j > b {
                b = j;
            }
        }
        sum += a * 10 + b;
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let g = Grid::from_bytes(input);
    let mut sum: u64 = 0;
    for row in g.rows() {
        let mut numbers: Vec<u64> = Vec::new();
        let mut i = 0;
        let mut remaining = 12;
        while remaining > 0 {
            let mut max = 0;
            for j in i..(row.len() - remaining + 1) {
                let joltage = (row[j] - b'0') as u64;
                if joltage > max {
                    max = joltage;
                    i = j + 1;
                }
            }
            numbers.push(max);
            remaining -= 1;
        }
        let mut number = 0;
        for n in numbers {
            number = number * 10 + n;
        }
        sum += number;
    }
    sum
}

#[test]
fn test_part_1() {
    let data = r"987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(part_1(data), 357);
}

#[test]
fn test_part_2() {
    let data = r"987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(part_2(data), 3121910778619);
}
