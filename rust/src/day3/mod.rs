use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct AOC2023_03 {
    file: &'static str,
    parts: Vec<Part>,
}

impl Runner for AOC2023_03 {
    fn name(&self) -> &str {
        "AOC2023_03"
    }

    fn parse(&mut self, filename: &'static str) -> () {
        self.file = filename;
        self.parts = Vec::new();
        let mut cur_number: Option<Part> = None;

        for (row, line) in self.file.lines().enumerate() {
            let line = format!("{}.", line);
            for (col, char) in line.chars().enumerate() {
                if char.is_digit(10) {
                    match cur_number {
                        Some(ref mut num) => {
                            num.add(char, row as i32, col as i32);
                        }
                        None => {
                            cur_number = Some(Part::new(char, row as i32, col as i32));
                        }
                    }
                } else {
                    match cur_number {
                        Some(num) => {
                            self.parts.push(num);
                            cur_number = None;
                        }
                        None => {}
                    }
                }
            }
        }
    }

    fn part1(&self) -> Result<i32, String> {
        let mut symbols: HashSet<(i32, i32)> = HashSet::new();
        for (row, line) in self.file.lines().enumerate() {
            let line = format!("{}.", line);
            for (col, char) in line.chars().enumerate() {
                if !char.is_digit(10) && char != '.' {
                    symbols.insert((row as i32, col as i32));
                }
            }
        }
        let mut total = 0;
        for part in &self.parts {
            if symbols.intersection(&part.adjacents).count() > 0 {
                total += part.value;
            }
        }
        Ok(total)
    }

    fn part2(&self) -> Result<i32, String> {
        let mut gears: HashSet<(i32, i32)> = HashSet::new();
        for (row, line) in self.file.lines().enumerate() {
            let line = format!("{}.", line);
            for (col, char) in line.chars().enumerate() {
                if char == '*' {
                    gears.insert((row as i32, col as i32));
                }
            }
        }
        let mut total = 0;
        for gear in gears {
            let mut count = 0;
            let mut numbers = Vec::new();
            for part in &self.parts {
                if part.adjacents.contains(&gear) {
                    numbers.push(part.value);
                    count += 1;
                }
            }
            if count == 2 {
                total += numbers[0] * numbers[1];
            }
        }
        Ok(total)
    }
}

#[derive(Debug)]
struct Part {
    value: i32,
    points: HashSet<(i32, i32)>,
    adjacents: HashSet<(i32, i32)>,
}

impl Part {
    fn new(value: char, x: i32, y: i32) -> Self {
        let mut points = HashSet::new();
        points.insert((x, y));
        Self {
            value: value.to_digit(10).unwrap() as i32,
            points,
            adjacents: Self::create_adjacents(x, y),
        }
    }

    fn add(&mut self, value: char, x: i32, y: i32) {
        self.value = self.value * 10 + value.to_digit(10).unwrap() as i32;
        self.points.insert((x, y));
        self.adjacents.extend(Self::create_adjacents(x, y));
    }

    fn create_adjacents(x: i32, y: i32) -> HashSet<(i32, i32)> {
        let mut adjacents = HashSet::new();
        adjacents.insert((x - 1, y - 1));
        adjacents.insert((x - 1, y));
        adjacents.insert((x - 1, y + 1));
        adjacents.insert((x, y - 1));
        adjacents.insert((x, y + 1));
        adjacents.insert((x + 1, y - 1));
        adjacents.insert((x + 1, y));
        adjacents.insert((x + 1, y + 1));
        adjacents
    }
}
