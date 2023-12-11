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

    fn part1(&mut self) -> Result<i32, String> {
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

    fn part2(&mut self) -> Result<i32, String> {
        let mut card_winnigs = Vec::new();
        let mut total_cards = Vec::new();

        for (index, ticket) in self.my_ticket.iter().enumerate() {
            let count = ticket.intersection(&self.winning_numbers[index]).count();
            card_winnigs.push(count as i32);
            total_cards.push(1 as i32);
        }

        for (card_index, winnings) in card_winnigs.iter().enumerate() {
            let mut ti = 0;
            while &ti < &total_cards[card_index] {
                let mut n: i32 = 0;
                while &n < winnings {
                    let i = n as usize;
                    if (card_index + i + 1) >= total_cards.len() {
                        break;
                    }
                    total_cards[card_index + i + 1] += 1;
                    n += 1;
                }
                ti += 1;
            }
        }
        Ok(total_cards.iter().sum())
    }
}
