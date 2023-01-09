pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

#[macro_export]
macro_rules! not_implemented {
    () => {
        return Err(format!("Not Implemented"))
    }
}

pub trait Solution {
    const DAY: u8;
    fn new(path: String) -> Self;
    fn part1(&self) -> Result<String, String>;
    fn part2(&self) -> Result<String, String>;
}
