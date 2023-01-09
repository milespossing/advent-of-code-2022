mod days;

fn try_solve<D: days::Solution>(p: String) {
    let solution = D::new(p);
    let pt1 = match solution.part1() {
        Ok(s) => s,
        Err(e) => format!("Error: {}", e),
    };
    let pt2 = match solution.part2() {
        Ok(s) => s,
        Err(e) => format!("Error: {}", e),
    };

    println!("Day {: >2}: Part 1: {}, Part 2: {}", D::DAY, pt1, pt2);
}

fn main() {
    println!("Day  1: {}", days::day01::solve_01().unwrap());
    println!("Day  2: {}", days::day02::solve(std::path::Path::new("./inputs")).unwrap());
    println!("Day  3: {}", days::day03::solve(std::path::Path::new("./inputs")).unwrap());
    println!("Day  4: {}", days::day04::solve(std::path::Path::new("./inputs")).unwrap());
    println!("Day  5: {}", days::day05::solve(std::path::Path::new("./inputs")).unwrap());
    try_solve::<days::day06::Solution>(format!("./inputs/day06.txt"));
}
