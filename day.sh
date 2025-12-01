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

    sed "s/DAY_NUMBER/$2/g" template/main.rs > "$rust_file"

    echo "
[[bin]]
name = \"$1_$2\"
path = \"src/$2.rs\"" >>"$1/Cargo.toml"

    touch "$input_file"
else
    echo "File $rust_file already exists"
fi
