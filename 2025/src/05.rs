fn main() {
    let input = include_str!("../input/05.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

struct Range {
    start: usize,
    end: usize,
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> usize {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
    let fresh_ranges = parse_ranges(fresh_ranges);

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<usize>().unwrap());
    let mut out = 0;

    for ingredient in ingredients {
        if fresh_ranges
            .iter()
            .any(|range| ingredient >= range.start && ingredient <= range.end)
        {
            out += 1;
        }
    }
    out
}

fn part_2(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(ranges);
    ranges.sort_by_key(|range| range.start);

    let mut new_ranges: Vec<Range> = vec![];
    for range in ranges {
        if let Some(last) = new_ranges.last_mut()
            && range.start <= last.end
        {
            last.end = last.end.max(range.end);
        } else {
            new_ranges.push(range);
        }
    }

    let mut out = 0;
    for range in new_ranges {
        out += range.end - range.start + 1;
    }
    out
}

#[test]
fn test_part_1() {
    let data = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part_1(data), 3);
}

#[test]
fn test_part_2() {
    let data = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part_2(data), 14);
}
