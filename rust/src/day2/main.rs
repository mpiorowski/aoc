use std::collections::HashMap;

const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

#[derive(Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn get_number_of_cubes(&self) -> i32 {
        match self {
            Color::Red => RED_CUBES,
            Color::Green => GREEN_CUBES,
            Color::Blue => BLUE_CUBES,
        }
    }
    fn get_str(&self) -> &str {
        match self {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        }
    }
}

fn extract_cubes(input: &str, i: usize) -> i32 {
    input
        .chars()
        .rev()
        .skip(input.len() - i + 1)
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn check_possible(input: &str) -> bool {
    for (i, _) in input.chars().enumerate() {
        for color in [Color::Red, Color::Green, Color::Blue].iter() {
            if input[i..].starts_with(color.get_str()) {
                let cubes = extract_cubes(input, i);
                if cubes > color.get_number_of_cubes() {
                    return false;
                }
            }
        }
    }
    true
}

fn get_max_cubes(input: &str) -> HashMap<&Color, i32> {
    let mut max_cubes = HashMap::<&Color, i32>::new();
    for i in 0..input.len() {
        for color in [Color::Red, Color::Green, Color::Blue].iter() {
            if input[i..].starts_with(color.get_str()) {
                let cubes = extract_cubes(input, i);
                if max_cubes.get(color).is_none() {
                    max_cubes.insert(color, cubes);
                } else {
                    let current_cubes = max_cubes.get(color).unwrap();
                    if cubes > *current_cubes {
                        max_cubes.insert(color, cubes);
                    }
                }
            }
        }
    }
    max_cubes
}

fn main() {
    println!("Starting day2!");

    let part1 = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if check_possible(line) {
                return (i + 1) as i32;
            }
            return 0;
        })
        .sum::<i32>();
    println!("Result part 1: {}", part1);

    let part2 = include_str!("./input.txt")
        .lines()
        .map(get_max_cubes)
        .map(|max_cubes| {
            let mut mult = 1;
            for (_, cubes) in max_cubes.iter() {
                mult *= cubes;
            }
            mult
        })
        .sum::<i32>();

    println!("Result part 2: {}", part2);
}
