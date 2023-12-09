use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Specify the path to your file
    let file_path = "/home/mrda/Documents/aoc23/day1b/input.txt";

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    #[allow(unused_assignments)]
    let mut two_digit = 0;
    let mut suma = 0;
    let mut first_digit = 0;
    let mut last_digit = 0;
    let numbers = ["one", "two","three","four","five", "six","seven","eight","nine"];

    // Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            for i in 0..line.len() {
                let chunk: String = line.chars().skip(i).take(5).collect();
                if chunk.len() == 5 {
                    if chunk.chars().next().unwrap().is_digit(10) {
                        first_digit = chunk.chars().nth(0).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(1).unwrap().is_digit(10) {
                        first_digit = chunk.chars().nth(1).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if let Some(word) =numbers.iter().find(|&&word| chunk.contains(word)) {
                        first_digit = word_to_number(word).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(2).unwrap().is_digit(10) {
                        first_digit = chunk.chars().nth(2).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(3).unwrap().is_digit(10) {
                        first_digit = chunk.chars().nth(3).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(4).unwrap().is_digit(10) {
                        first_digit = chunk.chars().nth(4).unwrap().to_digit(10).unwrap();
                        break;
                    }
                }
                else if chunk.len() < 3 {
                    if let Some(c) = chunk.chars().find(|c| c.is_digit(10)) {
                        first_digit = c.to_digit(10).unwrap();
                        break;
                    }
                }
                else {
                    if let Some(word) = numbers.iter().find(|&&word| chunk.contains(word)) {
                        first_digit = word_to_number(word).unwrap();
                        break;
                    }
                    else if let Some(c) = chunk.chars().find(|c| c.is_digit(10)) {
                        first_digit = c.to_digit(10).unwrap();
                        break;
                    }
                }
            }
            for i in (0..line.len()).rev() {
                let chunk: String = line.chars().skip(i).take(5).collect();
                if chunk.len() == 5 {
                    if chunk.chars().nth(4).unwrap().is_digit(10) {
                        last_digit = chunk.chars().nth(4).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(3).unwrap().is_digit(10) {
                        last_digit = chunk.chars().nth(3).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if let Some(word) =numbers.iter().find(|&&word| chunk.contains(word)) {
                        last_digit = word_to_number(word).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(2).unwrap().is_digit(10) {
                        last_digit = chunk.chars().nth(2).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(1).unwrap().is_digit(10) {
                        last_digit = chunk.chars().nth(1).unwrap().to_digit(10).unwrap();
                        break;
                    }
                    else if chunk.chars().nth(0).unwrap().is_digit(10) {
                        last_digit = chunk.chars().nth(0).unwrap().to_digit(10).unwrap();
                        break;
                    }
                }
                else if chunk.len() < 3 {
                    if let Some(c) = chunk.chars().rev().find(|c| c.is_digit(10)) {
                        last_digit = c.to_digit(10).unwrap();
                        break;
                    }
                }
                else {
                    if let Some(word) = numbers.iter().find(|&&word| chunk.contains(word)) {
                        last_digit = word_to_number(word).unwrap();
                        break;
                    }
                    else if let Some(c) = chunk.chars().rev().find(|c| c.is_digit(10)) {
                        last_digit = c.to_digit(10).unwrap();
                        break;
                    }
                }
            }
        }
        two_digit = first_digit*10 + last_digit;
        suma += two_digit;
    }
    println!("Suma: {suma}");

    Ok(())
}

fn word_to_number(word: &str) -> Option<u32> {
    match word.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
