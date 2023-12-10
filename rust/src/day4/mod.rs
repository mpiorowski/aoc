use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct AOC2023_04 {
    file: &'static str,
    winning_numbers: Vec<HashSet<i32>>,
    my_ticket: Vec<HashSet<i32>>,
}

impl Runner for AOC2023_04 {
    fn name(&self) -> &str {
        "AOC2023_04"
    }

    fn parse(&mut self, filename: &'static str) -> () {
        self.file = filename;
        let winning = self
            .file
            .lines()
            .map(|line| {
                line.chars()
                    .skip_while(|c| c != &':')
                    .skip(1)
                    .take_while(|c| c != &'|')
                    .skip_while(|c| c.is_whitespace())
                    .collect::<String>()
            })
            .map(|line| {
                line.split(" ")
                    .filter(|s| s != &"")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        self.winning_numbers = winning;

        let ticket = self
            .file
            .lines()
            .map(|line| {
                line.chars()
                    .skip_while(|c| c != &'|')
                    .skip(1)
                    .collect::<String>()
            })
            .map(|line| {
                line.split(" ")
                    .filter(|s| s != &"")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();
        self.my_ticket = ticket;
    }

    fn part1(&self) -> Result<i32, String> {
        let mut total = 0;
        for (index, ticket) in self.my_ticket.iter().enumerate() {
            let count = ticket.intersection(&self.winning_numbers[index]).count();
            if count == 0 {
                continue;
            }
            total += 2u32.pow(count as u32 - 1)
        }
        Ok(total as i32)
    }

    fn part2(&self) -> Result<i32, String> {
        todo!()
    }
}
