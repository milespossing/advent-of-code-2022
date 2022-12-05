use std::str::FromStr;

struct Elf {
    lower: u8,
    upper: u8,
}

impl FromStr for Elf {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: Vec<&str> = s.split('-').collect();
        let lower: u8 = u8::from_str_radix(arr[0], 10).unwrap();
        let upper: u8 = u8::from_str_radix(arr[1], 10).unwrap();
        Ok(Elf { lower, upper })
    }
}

pub fn solve(p: &std::path::Path) -> Result<String, &str> {
    let file = std::fs::read_to_string(p.join("day04.txt")).unwrap();
    let (r1, r2) = file.split('\n').filter(|&s| !s.is_empty()).map(|s| {
        let splt: Vec<&str> = s.split(',').collect();
        return (
            Elf::from_str(splt[0]).unwrap(),
            Elf::from_str(splt[1]).unwrap(),
        );
    })
    .fold((0,0), |(r1, r2), input| {
        let is_contained: i32 = match &input {
            (left, right) if left.lower <= right.lower && left.upper >= right.upper => 1,
            (left, right) if right.lower <= left.lower && right.upper >= left.upper => 1,
            _ => 0,
        };
        let overlap: i32 = match input {
            (left, right) if left.lower > right.upper || left.upper < right.lower => 0,
            _ => 1
        };
        (r1 + is_contained, r2 + overlap)
    });
    Ok(format!("Part 1: {}, Part 2: {}", r1, r2))
}
