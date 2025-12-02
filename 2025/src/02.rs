fn main() {
    let input = include_str!("../input/02.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn is_valid_part_1(n: i64) -> bool {
    let s = n.to_string();
    if s.len() % 2 == 1 {
        return true;
    }
    s[..s.len() / 2] != s[s.len() / 2..]
}

fn is_valid_part_2(n: i64) -> bool {
    let s = n.to_string();
    let s = s.as_bytes();
    for len in 1..=(s.len() / 2) {
        if s.len() % len != 0 {
            continue;
        }
        let mut chunks = s.chunks_exact(len);
        if let Some(first) = chunks.next() {
            if chunks.all(|chunk| chunk == first) {
                return false;
            }
        }
    }
    true
}

fn part_1(input: &str) -> i64 {
    let ranges = input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut out = 0;
    for (start, end) in ranges {
        for i in start..=end {
            if !is_valid_part_1(i) {
                out += i;
            }
        }
    }
    out
}

fn part_2(input: &str) -> i64 {
    let ranges = input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut out = 0;
    for (start, end) in ranges {
        for i in start..=end {
            if !is_valid_part_2(i) {
                out += i;
            }
        }
    }
    out
}

#[test]
fn test_part_1() {
    let data = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(part_1(data), 1227775554);
}

#[test]
fn test_part_2() {
    let data = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(part_2(data), 4174379265);
}
