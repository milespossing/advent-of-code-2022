mod days;

fn main() {
    println!("Day 01: {}", days::day01::solve_01().unwrap());
    println!("Day 02: {}", days::day02::solve(std::path::Path::new("./inputs")).unwrap());
}
