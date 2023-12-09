fn map_digits(lines: &str) -> (Vec<(i32, Vec<String>)>, Vec<String>) {
    let mut digits_map: Vec<(i32, Vec<String>)> = Vec::new();

    let mut symbols_indexes = Vec::new();

    for (row, line) in lines.lines().enumerate() {
        let mut was_digit = false;
        let mut digits = Vec::new();
        let mut digit_indexses = Vec::new();
        for (column, char) in line.chars().enumerate() {
            if is_symbol(char) {
                symbols_indexes.push(format!("{}||{}", row, column));
            }
            if char.is_digit(10) {
                was_digit = true;
                digits.push(char.to_digit(10).unwrap());
                add_adjacent_indexes(&mut digit_indexses, row, column);
            } else if was_digit {
                let all_digits_into_one_number = digits
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                digits_map.push((all_digits_into_one_number, digit_indexses));
                digits = Vec::new();
                digit_indexses = Vec::new();
                was_digit = false;
            }
        }
        if was_digit {
            let all_digits_into_one_number = digits
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            digits_map.push((all_digits_into_one_number, digit_indexses));
        }
    }
    (digits_map, symbols_indexes)
}

fn map_gears(lines: &str) -> (Vec<(i32, Vec<String>)>, Vec<Vec<String>>) {
    let mut digits_map: Vec<(i32, Vec<String>)> = Vec::new();
    let mut gears_indexes: Vec<Vec<String>> = Vec::new();

    for (row, line) in lines.lines().enumerate() {
        let mut was_digit = false;
        let mut digits = Vec::new();
        let mut gear_indexses = Vec::new();
        let mut digit_indexses = Vec::new();
        for (column, char) in line.chars().enumerate() {
            if is_gear(char) {
                add_adjacent_indexes(&mut gear_indexses, row, column);
                gears_indexes.push(gear_indexses.clone());
                gear_indexses = Vec::new();
            }
            if char.is_digit(10) {
                was_digit = true;
                digits.push(char.to_digit(10).unwrap());
                digit_indexses.push(format!("{}||{}", row, column));
            } else if was_digit {
                let all_digits_into_one_number = digits
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                digits_map.push((all_digits_into_one_number, digit_indexses));
                digits = Vec::new();
                digit_indexses = Vec::new();
                was_digit = false;
            }
        }
        if was_digit {
            let all_digits_into_one_number = digits
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            digits_map.push((all_digits_into_one_number, digit_indexses));
        }
    }
    (digits_map, gears_indexes)
}

fn add_adjacent_indexes(indexses: &mut Vec<String>, row: usize, column: usize) {
    let row: i32 = row as i32;
    let column: i32 = column as i32;
    indexses.push(format!("{}||{}", row - 1, column));
    indexses.push(format!("{}||{}", row - 1, column + 1));
    indexses.push(format!("{}||{}", row, column - 1));
    indexses.push(format!("{}||{}", row + 1, column - 1));
    indexses.push(format!("{}||{}", row - 1, column - 1));
    indexses.push(format!("{}||{}", row + 1, column));
    indexses.push(format!("{}||{}", row, column + 1));
    indexses.push(format!("{}||{}", row + 1, column + 1));
}

fn is_symbol(char: char) -> bool {
    char != '.' && !char.is_digit(10)
}

fn is_gear(char: char) -> bool {
    char == '*'
}

fn check_if_digital_have_adjacent_symbol(
    digital_indexes: &Vec<String>,
    symbols_indexes: &Vec<String>,
) -> bool {
    let mut has_adjacent_symbol = false;
    for index in digital_indexes {
        if symbols_indexes.contains(index) {
            has_adjacent_symbol = true;
            break;
        }
    }
    has_adjacent_symbol
}

fn get_multi_two_digits_next_to_gear(
    digits: &Vec<(i32, Vec<String>)>,
    gears: &Vec<Vec<String>>,
) -> Vec<i32> {
    let mut result = Vec::new();
    for gear in gears {
        let mut count = 0;
        let mut first_digit = 0;
        let mut second_digit = 0;
        for digit in digits {
            if gear.iter().any(|x| digit.1.contains(x)) {
                if count == 0 {
                    first_digit = digit.0;
                } else {
                    second_digit = digit.0;
                }
                count += 1;
            }
        }
        if count == 2 {
            let multiplication = first_digit * second_digit;
            result.push(multiplication);
        }
    }
    result
}

fn main() {
    println!("Starting day3!");

    let input = include_str!("./input.txt");

    let map = map_digits(input);
    let result = map
        .0
        .iter()
        .filter(|(_, value)| check_if_digital_have_adjacent_symbol(value, &map.1))
        .map(|(key, _)| key)
        .sum::<i32>();
    println!("result1: {}", result);

    let map = map_gears(input);
    let result = get_multi_two_digits_next_to_gear(&map.0, &map.1)
        .iter()
        .sum::<i32>();

    println!("result2: {}", result);
}
