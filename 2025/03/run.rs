// Advent of Code 2025 - Day 03
use std::io::{self, Read};
use std::panic::catch_unwind;

fn solve_1(input: &str) -> String {
    todo!("Part 1")
}

fn solve_2(input: &str) -> String {
    todo!("Part 2")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim().to_string();

    match catch_unwind(|| solve_1(&input)) {
        Ok(result) => println!("PART1:{}", result),
        Err(_) => println!("PART1:--"),
    }
    match catch_unwind(|| solve_2(&input)) {
        Ok(result) => println!("PART2:{}", result),
        Err(_) => println!("PART2:--"),
    }
}
