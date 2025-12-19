fn main() {
    println!("AOC 2025 - Day 2");

    let input = include_str!("input_1.txt");
    let lines: Vec<(i64, i64)> = input
        .lines()
        .map(|line| line.split(','))
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split = line.split('-').collect::<Vec<&str>>();
            (
                split[0].parse::<i64>().unwrap(),
                split[1].parse::<i64>().unwrap(),
            )
        })
        .collect();

    struct Counter {
        start: i64,
        end: i64,
        sum: i64,
    }

    impl Counter {
        fn new(start: i64, end: i64) -> Self {
            Counter { start, end, sum: 0 }
        }

        fn increment(&mut self) -> bool {
            if self.is_invalid() {
                self.sum += self.start;
            }
            if self.start < self.end {
                self.start += 1;
                return true;
            }
            false
        }

        fn is_invalid(&self) -> bool {
            let string_repr = self.start.to_string();
            let split_index = string_repr.len() / 2;
            let first_half = &string_repr[..split_index];
            let second_half = &string_repr[split_index..];
            if first_half == second_half {
                return true;
            }
            false
        }
    }

    let mut sum_all = 0;
    for (start, end) in &lines {
        let mut counter = Counter::new(*start, *end);
        while counter.increment() {}
        sum_all += counter.sum;
    }
    println!("Invalid sum: {}", sum_all);

    struct Repeater {
        start: i64,
        end: i64,
        sum: i64,
    }

    impl Repeater {
        fn new(start: i64, end: i64) -> Self {
            Repeater { start, end, sum: 0 }
        }

        fn increment(&mut self) -> bool {
            if self.is_invalid() {
                self.sum += self.start;
            }
            if self.start < self.end {
                self.start += 1;
                return true;
            }
            false
        }

        fn is_invalid(&self) -> bool {
            let s = self.start.to_string();
            let len = s.len();
            for prefix_len in 1..=len / 2 {
                if len % prefix_len == 0 {
                    let prefix = &s[..prefix_len];
                    if prefix.repeat(len / prefix_len) == s {
                        return true;
                    }
                }
            }
            false
        }
    }

    let mut sum_all_repeater = 0;
    for (start, end) in lines {
        let mut repeater = Repeater::new(start, end);
        while repeater.increment() {}
        sum_all_repeater += repeater.sum;
    }
    println!("Repeater sum: {}", sum_all_repeater);
}
