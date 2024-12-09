pub fn part_1(input: &str) -> usize {
    let mut data = Vec::new();
    let mut is_free = false;
    let mut i = 0;
    for c in input.bytes() {
        for _ in 0..(c - b'0') {
            if is_free {
                data.push(-1);
            } else {
                data.push(i as isize);
            }
        }
        if !is_free {
            i += 1;
        }
        is_free = !is_free;
    }

    let mut i = 0;
    let mut j = data.len() - 1;

    while i < j {
        while i < j && data[i] != -1 {
            i += 1;
        }

        while j > i && data[j] == -1 {
            j -= 1;
        }

        if i < j {
            data.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    let mut result = 0;

    for (i, c) in data.into_iter().enumerate() {
        match c {
            -1 => (),
            c => result += i * c as usize,
        }
    }

    result
}

pub fn part_2(input: &str) -> usize {
    let mut data = Vec::new();
    let mut is_free = false;
    let mut i = 0;
    for c in input.bytes() {
        for _ in 0..(c - b'0') {
            if is_free {
                data.push(-1);
            } else {
                data.push(i as isize);
            }
        }
        if !is_free {
            i += 1;
        }
        is_free = !is_free;
    }

    let mut j = data.len() - 1;

    while j > 0 {
        while data[j] == -1 {
            j -= 1;
        }
        let id = data[j];
        let mut j_start = j - 1;
        while j_start > 0 && data[j_start] == id {
            j_start -= 1;
        }
        j_start += 1;
        let file = j + 1 - j_start;

        let mut i = 0;
        while i < j {
            while i < j && data[i] != -1 {
                i += 1;
            }

            let mut i_end = i;
            while i_end < j && data[i_end] == -1 {
                i_end += 1;
            }

            let free = i_end - i;
            if free < file {
                i += free + 1;
                continue;
            }

            for k in 0..file {
                data.swap(i + k, j_start + k);
            }
            i += file;
            j -= file;
        }

        j = j_start - 1;
    }

    let mut result = 0;

    for (i, c) in data.into_iter().enumerate() {
        if c != -1 {
            result += i * c as usize;
        }
    }

    result
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
    let data = r"2333133121414131402";
    assert_eq!(part_1(data), 1928);
}

#[test]
fn test_part_2() {
    let data = r"2333133121414131402";
    assert_eq!(part_2(data), 2858);
}
