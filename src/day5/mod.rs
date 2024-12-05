fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = input.lines();

    let mut rules = Vec::<(i32, i32)>::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("|");
        let (a, b) = (
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        );
        rules.push((a, b));
    }

    let mut orderings = Vec::<Vec<i32>>::new();
    while let Some(line) = lines.next() {
        orderings.push(line.split(",").map(|x| x.parse().unwrap()).collect());
    }
    (rules, orderings)
}

fn is_valid_ordering(ordering: &[i32], rules: &[(i32, i32)]) -> bool {
    for (a, b) in rules {
        if let Some(p_a) = ordering.iter().position(|&x| x == *a) {
            if let Some(p_b) = ordering.iter().position(|&x| x == *b) {
                if p_a > p_b {
                    return false;
                }
            }
        }
    }
    true
}

fn make_ordering_valid(ordering: &mut [i32], rules: &[(i32, i32)]) -> bool {
    let mut was_invalid = false;
    while !is_valid_ordering(ordering, rules) {
        for (a, b) in rules {
            if let Some(p_a) = ordering.iter().position(|&x| x == *a) {
                if let Some(p_b) = ordering.iter().position(|&x| x == *b) {
                    if p_a > p_b {
                        ordering.swap(p_a, p_b);
                        was_invalid = true;
                    }
                }
            }
        }
    }
    was_invalid
}

pub fn part_1(input: &str) -> i32 {
    let (rules, orderings) = parse_input(input);

    let mut sum = 0;
    for ordering in orderings {
        if is_valid_ordering(&ordering, &rules) {
            sum += ordering[ordering.len() / 2];
        }
    }
    sum
}

pub fn part_2(input: &str) -> i32 {
    let (rules, orderings) = parse_input(input);

    let mut sum = 0;
    for mut ordering in orderings {
        if make_ordering_valid(&mut ordering, &rules) {
            sum += ordering[ordering.len() / 2];
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
    let data = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part_1(data), 143);
}

#[test]
fn test_part_2() {
    let data = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part_2(data), 123);
}
