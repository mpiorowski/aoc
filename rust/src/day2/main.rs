const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn get_max_cubes(&self) -> i32 {
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
                if cubes > color.get_max_cubes() {
                    println!(
                        "Too many {} cubes: {} > {}",
                        color.get_str(),
                        cubes,
                        color.get_max_cubes()
                    );
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    println!("Starting day2!");

    let input = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if check_possible(line) {
                return (i + 1) as i32;
            }
            return 0;
        })
        .sum::<i32>();

    println!("Result: {}", input);
}
