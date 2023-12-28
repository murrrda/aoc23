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

pub fn part_1(input: &str) -> i64 {
    let times = read_times(input);
    let distances = read_distances(input);

    let races = form_races(times, distances);

    let mut counts: Vec<i64> = Vec::new();

    for race in races {
        let x = find_roots_quadratic(1 as f64, -race.time as f64, race.distance as f64);
        if let Roots::Two([first_el, second_el]) = x {
            let count = second_el.floor() as i64 - first_el.floor() as i64;
            counts.push(count);
        } else {
            println!("Unexpected result");
        }
    }

    let mut ret_val: i64 = 1;
    for num in counts {
        println!("{:?}", num);
        ret_val *= num;
    }

    return ret_val;
}
