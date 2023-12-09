use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Specify the path to your file
    let file_path = "input.txt";

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    #[allow(unused_assignments)]
    let mut two_digit = 0;
    let mut suma = 0;

    // Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            let left_digit = line.chars().find(|c| c.is_digit(10));
            let right_digit = line.chars().rev().find(|c| c.is_digit(10));

            if let (Some(right_digit), Some(left_digit)) = (right_digit, left_digit) {
                let left_value = left_digit.to_digit(10).unwrap_or(0);
                let right_value = right_digit.to_digit(10).unwrap_or(0);

                two_digit = left_value * 10 + right_value;
                suma += two_digit;
            }
        }

    }
    println!("Suma: {suma}");

    Ok(())
}
