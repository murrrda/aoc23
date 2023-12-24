#!/bin/bash

if [ -z "$1" ]; then
    echo "$0 <day_number>"
    exit 1
fi

day_number=$1

cargo new day$day_number

url="https://adventofcode.com/2023/day/$day_number/input"
output_file="day$day_number/input.txt"
session=$(cat cookie.txt)

curl -s --cookie "session=$session" "$url" > "$output_file"

echo $day_number
