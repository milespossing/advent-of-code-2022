mod days;

fn main() {
    println!("Day 01: {}", days::day01::solve_01().unwrap());
    println!("Day 02: {}", days::day02::solve(std::path::Path::new("./inputs")).unwrap());
    println!("Day 03: {}", days::day03::solve(std::path::Path::new("./inputs")).unwrap());
    let (d4_1, d4_2) = days::day04::solve(std::path::Path::new("./inputs")).unwrap();
    println!("Day 04: pt1: {} pt2: {}", d4_1, d4_2);
}
