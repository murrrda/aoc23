use roots::Roots;
use roots::find_roots_quadratic;

#[derive(Debug)]
pub struct Race {
    time: i64,
    distance: i64,
}

fn read_times(input: &str) -> Vec<i64> {
    let times: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|time| time.parse().unwrap())
        .collect();
    times
}
fn read_distances(input: &str) -> Vec<i64> {
    let distances: Vec<i64> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|time| time.parse().unwrap())
        .collect();
    distances
}

fn form_races(times: Vec<i64>, distances: Vec<i64>) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    for (index, _member) in times.iter().enumerate() {
        let race = Race {
            time: times[index],
            distance: distances[index],
        };
        races.push(race)
    }
    races
}

fn find_count_for_race(race: Race) -> i64 {
    let x = find_roots_quadratic(1 as f64, -race.time as f64, race.distance as f64);
    let mut count: i64 = 0;
    
    if let Roots::Two([first_el, second_el]) = x {
        count = second_el.floor() as i64 - first_el.floor() as i64;
    } else {
        println!("Unexpected result");
    }

    count
}

fn parse_time(input: &str) -> i64 {
    let num_vec: Vec<&str> =input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();

    let mut num_str = String::from("");
    for num in num_vec {
        num_str.push_str(num);
    }

    num_str.parse().unwrap()
}

fn parse_distance(input: &str) -> i64 {
    let num_vec: Vec<&str> =input
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();

    let mut num_str = String::from("");
    for num in num_vec {
        num_str.push_str(num);
    }

    num_str.parse().unwrap()
}

pub fn part_1(input: &str) -> i64 {
    let times = read_times(input);
    let distances = read_distances(input);
    let races = form_races(times, distances);
    let mut counts: Vec<i64> = Vec::new();

    for race in races {
        counts.push(find_count_for_race(race));
    }

    let mut ret_val: i64 = 1;
    for num in counts {
        ret_val *= num;
    }

    return ret_val;
}

pub fn part_2(input: &str) -> i64 {
    let time = parse_time(input);
    let distance = parse_distance(input);
    let race = Race {
        time,
        distance,
    };

    find_count_for_race(race)
}
