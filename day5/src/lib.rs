use std::{str::FromStr, ops::Range};
use core::str::Lines;

#[derive(Debug, Default)]
pub struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let ret_val = Almanac {
            seeds: 
                line
                .split_once(": ")
                .unwrap().1
                .split(" ")
                .map(|seed| 
                    seed.parse().unwrap()
                )
                .collect(),
            mappings:
                Vec::new(),
        };
        Ok(ret_val)
    }
}

impl Almanac {
    pub fn create_mappings(&mut self, mut lines_iter: Lines<'_>) {
        lines_iter.next();
        lines_iter.next();

        let mut cur_map = Mapping::default();
        for line in lines_iter {
            if line.contains(":") {
                self.mappings.push(cur_map);
                cur_map = Mapping::default();
                continue;
            }
            if line.is_empty() {
                continue;
            }
            let nums: Vec<i64> = line.split(" ").map(|num| num.parse::<i64>().unwrap()).collect();
            cur_map.add_mapping(nums[0], nums[1], nums[2]);
        }
        self.mappings.push(cur_map);
    }

}

#[derive(Debug, Default)]
pub struct SingeMap {
    range: Range<i64>,
    diff: i64,
}

#[derive(Debug,Default)]
pub struct Mapping {
    map: Vec<SingeMap>,
}

impl Mapping {
    fn add_mapping(&mut self, dest: i64, src: i64, range_len: i64) {
        self.map.push(SingeMap {
            range: Range {
                start: src,
                end: src + range_len,
            },
            diff: dest - src,
        })
    }
    fn apply_map(&self, seed: i64) -> i64 {
        for map in self.map.iter() {
            if map.range.contains(&seed) {
                return seed + map.diff;
            }
        }

        seed
    }
}

pub fn part_1(input: &str) -> i64 {
    let mut lines_iter = input.lines();
    let mut almanac = Almanac::from_str(lines_iter.next().unwrap()).unwrap();

    almanac.create_mappings(lines_iter);

    let mut min = i64::MAX;
    for seed in &almanac.seeds {
        let mut current_seed = *seed;

        for map in almanac.mappings.iter() {
            current_seed = map.apply_map(current_seed);
        }
        min = min.min(current_seed);
    }

    min
}
pub fn part_2(input: &str) -> i64 {
    let mut lines_iter = input.lines();
    let mut almanac = Almanac::from_str(lines_iter.next().unwrap()).unwrap(); 

    almanac.create_mappings(lines_iter);

    let mut min = i64::MAX;

    for seed_range in almanac.seeds.chunks(2) {
        for seed in seed_range[0]..seed_range[0]+seed_range[1] {
            let mut current_seed = seed;

            for map in &almanac.mappings {
                current_seed = map.apply_map(current_seed);
            }
            min = min.min(current_seed);
        }
    }

    min
}
