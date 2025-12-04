fn main() {
    println!("AOC 2025 - Day 1");

    let input = include_str!("day_1/input_1.txt");
    let lines: Vec<&str> = input.lines().collect();

    let moves = lines
        .iter()
        .map(|line| {
            let dir = &line[0..1];
            let dist: i32 = line[1..].parse().unwrap();
            (dir, dist)
        })
        .collect::<Vec<(&str, i32)>>();

    let start_position = 50;

    struct Dial {
        position: i32,
        size: i32,
    }

    impl Dial {
        fn new(size: i32, start_position: i32) -> Self {
            Dial {
                position: start_position,
                size,
            }
        }

        fn turn(&mut self, dir: &str, dist: i32) {
            match dir {
                "L" => {
                    self.position = (self.position - dist).rem_euclid(self.size);
                }
                "R" => {
                    self.position = (self.position + dist).rem_euclid(self.size);
                }
                _ => panic!("Invalid direction"),
            }
        }

        fn left(&mut self) {
            self.position -= 1;
            if self.position < 0 {
                self.position = self.size - 1;
            }
        }

        fn right(&mut self) {
            self.position += 1;
            if self.position >= self.size {
                self.position = 0;
            }
        }
    }

    let mut dial = Dial::new(100, start_position);
    let mut counter = 0;
    for (dir, dist) in &moves {
        dial.turn(dir, *dist);
        if dial.position == 0 {
            counter += 1;
        }
    }
    println!("Counter 1: {}", counter);

    let mut dial2 = Dial::new(100, start_position);
    let mut counter2 = 0;
    for (dir, dist) in &moves {
        for _ in 0..*dist {
            match *dir {
                "L" => dial2.left(),
                "R" => dial2.right(),
                _ => panic!("Invalid direction"),
            }
            if dial2.position == 0 {
                counter2 += 1;
            }
        }
    }
    println!("Counter 2: {}", counter2);
}
