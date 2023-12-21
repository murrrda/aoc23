use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn is_dot(char: char) -> bool {
    char == '.'
}
pub fn is_symbol(char: char) -> bool {
    !char.is_digit(10) && char != '.' && char != '\n'
}

pub fn read_to_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a matrix to store characters
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Read lines from the file and populate the matrix
    for line in reader.lines() {
        let row: Vec<char> = line?.chars().collect();
        matrix.push(row);
    }

    Ok(matrix)
}

pub fn look_for_symbols(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x > 0 && y > 0 {
        if is_symbol(matrix[x-1][y-1]) {
            return true;
        }
        else if is_symbol(matrix[x-1][y]) {
            return true;
        }
        else if (y+1) < matrix[x-1].len() && is_symbol(matrix[x-1][y+1]){
            return true;
        }
        else if is_symbol(matrix[x][y-1]){
            return true;
        }
        else if (y+1) < matrix[x].len() && is_symbol(matrix[x][y+1]){
            return true;
        }
        else if (x+1) < matrix.len() && is_symbol(matrix[x+1][y-1]){
            return true;
        }
        else if (x+1) < matrix.len()&& is_symbol(matrix[x+1][y])  {
            return true;
        }
        else if (x+1) < matrix.len() && (y+1) < matrix[x+1].len() && is_symbol(matrix[x+1][y+1]) {
            return true;
        }
        else {
            return false;
        }
    }
    else if  x == 0 && y > 0{
        if is_symbol(matrix[x][y-1]){
            return true;
        }
        else if (y+1) < matrix[x].len()&& is_symbol(matrix[x][y+1])  {
            return true;
        }
        else if (x+1) < matrix.len() && is_symbol(matrix[x+1][y-1]) {
            return true;
        }
        else if (x+1) < matrix.len() && is_symbol(matrix[x+1][y]) {
            return true;
        }
        else if (x+1) < matrix.len() &&  (y+1) < matrix[x+1].len() && is_symbol(matrix[x+1][y+1]) {
            return true;
        }
        else {
            return false;
        }
    }
    else if x > 0 && y == 0 {
        if is_symbol(matrix[x-1][y]) {
            return true;
        }
        else if (y+1) < matrix[x-1].len() && is_symbol(matrix[x-1][y+1]) {
            return true;
        }
        else if (y+1) < matrix[x].len() && is_symbol(matrix[x][y+1]) {
            return true;
        }
        else if (x+1) < matrix.len() && is_symbol(matrix[x+1][y]) {
            return true;
        }
        else if (x+1) < matrix.len() && (y+1) < matrix[x+1].len() && is_symbol(matrix[x+1][y+1]) {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        if (y+1) < matrix[x].len() && is_symbol(matrix[x][y+1]) {
            return true;
        }
        else if (x+1) < matrix.len() && is_symbol(matrix[x+1][y]) {
            return true;
        }
        else if (x+1) < matrix.len() && (y+1) < matrix[x+1].len() && is_symbol(matrix[x+1][y+1]) {
            return true;
        }
        else {
            return false;
        }
    }
}

pub fn check_surrounding(matrix: &Vec<Vec<char>>, x: usize, y: usize, result: &mut (bool, u32)) {
    result.1 = result.1 * 10 + matrix[x][y].to_digit(10).unwrap();

    // looking for symbols first
    if look_for_symbols(matrix, x, y) {
        result.0 = true;
    }
    if (y+1) < matrix[x].len() && matrix[x][y+1].is_digit(10) {
        check_surrounding(matrix, x, y+1, result);
    }
}

pub fn part_1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = read_to_matrix(input).unwrap();

    let mut sum = 0;
    for (row_index, row) in matrix.iter().enumerate() {

        let mut char_iter = row.iter().enumerate();

        while let Some((char_index, &ch)) = char_iter.next() {
            let mut result: (bool, u32) = (false, 0);

            if ch.is_numeric() {
                check_surrounding(&matrix, row_index, char_index, &mut result);
                let num_len = result.1.to_string().len();
                if result.0 {
                    sum += result.1;
                }
                for _ in 0..num_len-1 {
                    char_iter.next();
                }
            }

        }

    }

    sum
}
