use day5::{part_1, part_2};
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let now_1 = Instant::now();
    print!("Answer Part 1: {}", part_1(input));
    let elapsed_1 = now_1.elapsed();
    println!(" -Time: {:?}", elapsed_1);

    let now_1 = Instant::now();
    print!("Answer Part 2: {}", part_2(input));
    let elapsed_1 = now_1.elapsed();
    println!(" -Time: {:?}", elapsed_1);
}
