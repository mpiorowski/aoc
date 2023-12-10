mod day1;
mod day2;
mod day3;

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
    let mut aoc = day3::AOC2023_03::default();
    let input_test1 = include_str!("./day3/input_t1.txt");
    let input_test2 = include_str!("./day3/input_t2.txt");
    let input_p = include_str!("./day3/input_p.txt");
    print_runner(&aoc);
    aoc.parse(input_test1);
    let r = aoc.part1();
    println!("Result part1 test: {:?}", r);
    aoc.parse(input_p);
    let r = aoc.part1();
    println!("Result part1: {:?}", r);
    // aoc.parse(input_test2);
    // let r = aoc.part2();
    // println!("Result part2 test: {:?}", r);
    // aoc.parse(input_p);
    // let r = aoc.part2();
    // println!("Result part2: {:?}", r);
}
