fn find_first_and_last_number_in_string(input: &str) -> i32 {
    let mut first: char = '0';
    let mut last: char = '0';
    for c in input.chars() {
        if c.is_digit(10) {
            first = c;
            break;
        }
    }
    for c in input.chars().rev() {
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
    println!("Starting day1!");

    let input = include_str!("./d1_p1_t.txt")
        .lines()
        .map(find_first_and_last_number_in_string)
        .sum::<i32>();

    println!("Result: {}", input);
}
