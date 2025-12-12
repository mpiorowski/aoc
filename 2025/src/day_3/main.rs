fn main() {
    println!("AOC 2025 - Day 3");

    let input = include_str!("input_1.txt");
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i64)
                .collect::<Vec<i64>>()
        })
        .collect();

    println!("{:?}", lines);

    struct Travel {
        first: i64,
        second: i64,
        position: usize,
        width: usize,
        path: Vec<i64>,
    }

    impl Travel {
        fn new(path: Vec<i64>) -> Self {
            Travel {
                first: path[0],
                second: path[1],
                position: 0,
                width: path.len(),
                path: path,
            }
        }

        fn r#move(&mut self) {
            self.position += 1;
            if self.position + 2 > self.width {
                return;
            }

            if self.path[self.position + 1] > self.second {
                self.second = self.path[self.position + 1];
            }

            if self.path[self.position] > self.first {
                self.first = self.path[self.position];
                self.second = self.path[self.position + 1];
            }

            self.r#move()
        }
    }

    let mut joltage: Vec<i64> = vec![];
    for line in &lines {
        let mut travel = Travel::new(line.to_vec());
        travel.r#move();
        println!("first: {:?}, second: {:?}", travel.first, travel.second);
        joltage.push(travel.first * 10 + travel.second)
    }

    println!("Sum_1: {:?}", joltage.iter().sum::<i64>());

    struct TravelL {
        values: Vec<i64>,
        index: usize,
        current: usize,
        position: usize,
        width: usize,
        path: Vec<i64>,
    }

    impl TravelL {
        fn new(path: Vec<i64>) -> Self {
            TravelL {
                values: vec![
                    path[0], path[1], path[2], path[3], path[4], path[5], path[6], path[7],
                    path[8], path[9], path[10], path[11],
                ],
                index: 0,
                current: 0,
                position: 0,
                width: path.len(),
                path: path,
            }
        }

        fn r#move(&mut self) {
            self.current += 1;
            if self.current + 11 - self.index >= self.width {
                return;
            }

            if self.path[self.current] > self.values[self.index] {
                self.values[self.index] = self.path[self.current];
                self.position = self.current;
            }
            self.r#move()
        }

        fn next(&mut self) {
            let new_path = &self.path[self.position + 1..];
            if new_path.len() < 11 - self.index {
                return;
            }
            self.index += 1;
            self.current = 0;
            self.position = 0;
            self.path = new_path.to_vec();
            self.values[self.index] = self.path[0];
            self.width = self.path.len();
            self.r#move();
        }
    }

    let mut total: i64 = 0;
    for line in lines {
        let mut travel = TravelL::new(line);
        travel.r#move();
        for _ in 0..11 {
            travel.next();
        }
        total += travel.values.iter().fold(0i64, |acc, &d| acc * 10 + d);
        println!("values: {:?}", travel.values);
    }

    println!("Sum_2: {}", total);
}
