#!/bin/sh
set -e

if [ "$#" -ne 2 ]; then
    echo "Usage: day.sh <year> <day>"
    exit 1
fi

rust_file="$1/src/$2.rs"
input_file="$1/input/$2.txt"

if [ ! -f "$rust_file" ]; then
    echo "Creating new file $1/src/$2.rs"
    echo "Creating new file $1/input/$2.txt"

    echo "fn main() {
    let input = include_str!(\"../input/$2.txt\");
    println!(\"Part 1: {}\", part_1(input));
    println!(\"Part 2: {}\", part_2(input));
}

fn part_1(input: &str) -> i32 {
    0
}

fn part_2(input: &str) -> i32 {
    0
}

#[test]
fn test_part_1() {
    let data = r\"\";
    assert_eq!(part_1(data), 0);
}

#[test]
fn test_part_2() {
    let data = r\"\";
    assert_eq!(part_2(data), 0);
}
" >"$rust_file"

    echo "
[[bin]]
name = \"$2\"
path = \"src/$2.rs\"" >>"$1/Cargo.toml"

    touch "$input_file"
else
    echo "File $rust_file already exists"
fi
