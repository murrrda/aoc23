use std::str::FromStr;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Default)]
pub struct NumberOfEach {
    pub red: usize,
    pub green: usize,
    pub blue: usize
}

impl FromStr for NumberOfEach {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = NumberOfEach::default();
        let subsets = s.split(", ");
        for subset in subsets {
            let mut subset_part_iter = subset.split_ascii_whitespace();
            let number: usize = subset_part_iter.next().unwrap_or("0").parse().unwrap_or(0);
            let word = subset_part_iter.next();
            if let Some(word) = word {
                match word {
                    "red" =>result.red = number,
                    "green" =>result.green = number,
                    "blue" =>result.blue = number,
                    _ => {}
                }
            }
        }
        Ok(result)
    }
}

#[derive(Default)]
pub struct Game {
    pub id: usize,
    pub cubes: Vec<NumberOfEach>
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Game::default();
        let mut part_iter = s.split(": ");
        result.id = part_iter
            .next()
            .unwrap_or("Game 1")
            .strip_prefix("Game ")
            .unwrap_or("1")
            .parse()
            .unwrap_or(1);
        let subsets = part_iter.next().unwrap().split("; ");
        for subset in subsets {
            result.cubes.push(NumberOfEach::from_str(subset).unwrap());
        }
        //result.cubes = part_iter
        //    .next()
        //    .unwrap_or("0 red, 0 green, 0 blue")
        //    .split("; ")
        //    .map(|string| {
        //        NumberOfEach::from_str(string)
        //            .unwrap_or(NumberOfEach::default())
        //    })
        //    .collect::<Vec<NumberOfEach>>();
        Ok(result)
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| 
            game.cubes
                .iter()
                .all(|number_of_each| 
                    number_of_each.red <= MAX_RED
                        && number_of_each.blue <= MAX_BLUE
                        && number_of_each.green <= MAX_GREEN
                )
        )
        .map(|game| game.id)
        .sum::<usize>()
}
