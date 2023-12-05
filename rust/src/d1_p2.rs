#[derive(PartialEq)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    None,
}

impl Number {
    fn search_for_number(input: &str) -> Number {
        match input {
            input if input.starts_with("one") => Number::One,
            input if input.starts_with("two") => Number::Two,
            input if input.starts_with("three") => Number::Three,
            input if input.starts_with("four") => Number::Four,
            input if input.starts_with("five") => Number::Five,
            input if input.starts_with("six") => Number::Six,
            input if input.starts_with("seven") => Number::Seven,
            input if input.starts_with("eight") => Number::Eight,
            input if input.starts_with("nine") => Number::Nine,
            _ => Number::None,
        }
    }

    fn change_number_to_char(number: Number) -> char {
        match number {
            Number::One => '1',
            Number::Two => '2',
            Number::Three => '3',
            Number::Four => '4',
            Number::Five => '5',
            Number::Six => '6',
            Number::Seven => '7',
            Number::Eight => '8',
            Number::Nine => '9',
            Number::None => '0',
        }
    }
}

fn find_first_and_last_number_in_string(input: &str) -> i32 {
    let mut first: char = '0';
    let mut last: char = '0';
    for (i, c) in input.chars().enumerate() {
        let number = Number::search_for_number(&input[i..]);
        if number != Number::None {
            first = Number::change_number_to_char(number);
            break;
        }
        if c.is_digit(10) {
            first = c;
            break;
        }
    }
    for (i, c) in input.chars().rev().enumerate() {
        let number = Number::search_for_number(&input[input.len() - i..]);
        if number != Number::None {
            last = Number::change_number_to_char(number);
            break;
        }
        if c.is_digit(10) {
            last = c;
            break;
        }
    }

    let mut s = String::new();
    s.push(first);
    s.push(last);
    return s.parse::<i32>().unwrap();
}

fn main() {
    println!("Starting day1 part2!");

    let input = include_str!("./d1.txt")
        .lines()
        .map(find_first_and_last_number_in_string)
        .sum::<i32>();

    println!("Result: {}", input);
}
