use crate::Runner;

pub struct AOC2023_01 {
    pub file: &'static str,
}

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

impl Runner for AOC2023_01 {
    fn name(&self) -> &str {
        "AOC2023_01"
    }

    fn parse(&mut self, file: &'static str) -> () {
        self.file = file;
    }

    fn part1(&self) -> Result<i32, String> {
        let lines = self.file.lines();
        let mut total = 0;
        for line in lines {
            let numbers = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c as i32 - '0' as i32)
                .collect::<Vec<_>>();
            let first_number = numbers.iter().nth(0).unwrap();
            let last_number = numbers.iter().last().unwrap();
            total += (first_number * 10) + last_number;
        }
        Ok(total)
    }

    fn part2(&self) -> Result<i32, String> {
        let lines = self.file.lines();
        let mut total = 0;
        for line in lines {
            let mut numbers = Vec::new();
            for (index, char) in line.chars().enumerate() {
                if char.is_digit(10) {
                    numbers.push(char as i32 - '0' as i32);
                } else {
                    let number = Number::search_for_number(&line[index..]);
                    if number == Number::None {
                        continue;
                    }
                    let number = Number::change_number_to_char(number);
                    numbers.push(number as i32 - '0' as i32);
                }
            }
            let first_number = numbers.iter().nth(0).unwrap();
            let last_number = numbers.iter().last().unwrap();
            total += (first_number * 10) + last_number;
        }
        Ok(total)
    }
}
