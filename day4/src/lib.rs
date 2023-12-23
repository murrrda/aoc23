use std::str::FromStr;

pub struct Numbers {
    pub winning_numbers: Vec<usize>,
    pub my_numbers: Vec<usize>,
}

impl FromStr for Numbers {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        line.split(": ").next(); // card part
        let mut numbers_part = line.split("|");
        let result = Numbers {
            winning_numbers: 
                numbers_part
                .next()
                .unwrap_or_default()
                .split_whitespace()
                .map(|string| string.parse::<usize>().unwrap_or_default())
                .collect(),
            my_numbers:
                numbers_part
                .next()
                .unwrap_or_default()
                .split_whitespace()
                .map(|string| string.parse::<usize>().unwrap_or_default())
                .collect(),
        };

        Ok(result)
    }
}

pub fn calculate_points(winner_count: Vec<usize>) -> usize {
    let points_won = winner_count
        .iter()
        .map(|&number| {
            if number == 0 {
                0
            } else if number == 1 {
                1
            } else {
                2usize.pow((number - 1usize) as u32)
            }
        })
        .sum();

    points_won
}

pub fn calculate_scratchcards(winner_count: Vec<usize>) -> usize {
    let mut result_scratchcards: Vec<usize> = winner_count.iter().map(|&_| 0).collect();
    for (pos, &number) in winner_count.iter().enumerate() {
        if number != 0 {
            result_scratchcards[pos] += 1;
            for i in 1..=number {
                if pos+i < result_scratchcards.len() {
                    result_scratchcards[pos+i] += result_scratchcards[pos];
                }
            }
        }
        else if number == 0 && result_scratchcards[pos] != 0 {
            result_scratchcards[pos] += 1;
        }
        else {
            result_scratchcards[pos] = 1;
        }
    }

    result_scratchcards
        .iter()
        .sum()
}

pub fn part_1(input: &str) -> usize {
    let winner_count: Vec<usize> = input   
        .lines()
        .map(|line| Numbers::from_str(line).unwrap())
        .map(|numbers| {
            let count = numbers
                .my_numbers
                .iter()
                .filter(|&number| numbers.winning_numbers.contains(number))
                .count();
            count
        })
        .collect();

    calculate_points(winner_count)
}

pub fn part_2(input: &str) -> usize {
    let winner_count: Vec<usize> = input   
        .lines()
        .map(|line| Numbers::from_str(line).unwrap())
        .map(|numbers| {
            let count = numbers
                .my_numbers
                .iter()
                .filter(|&number| numbers.winning_numbers.contains(number))
                .count();
            count
        })
        .collect();

    calculate_scratchcards(winner_count)
}
