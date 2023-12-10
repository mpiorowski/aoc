mod day1;
mod day2;

trait Runner {
    fn name(&self) -> &str;
    fn parse(&mut self, filename: &'static str) -> ();
    fn part1(&self) -> Result<i32, String>;
    fn part2(&self) -> Result<i32, String>;
}

fn print_runner(r: &dyn Runner) {
    println!("Runner: {}", r.name());
}

fn main() {
    println!("Hello, world!");
    let mut aoc = day2::AOC2023_02 { file: "" };
    print_runner(&aoc);
    aoc.parse(include_str!("./day2/input_t1.txt"));
    let r = aoc.part1();
    println!("Result part1 test: {:?}", r);
    aoc.parse(include_str!("./day2/input_p.txt"));
    let r = aoc.part1();
    println!("Result part1: {:?}", r);
    aoc.parse(include_str!("./day2/input_t2.txt"));
    let r = aoc.part2();
    println!("Result part2 test: {:?}", r);
    aoc.parse(include_str!("./day2/input_p.txt"));
    let r = aoc.part2();
    println!("Result part2: {:?}", r);
}
