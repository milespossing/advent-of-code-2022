use std::str::FromStr;

struct Elf {
    lower: u8,
    upper: u8
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
    let result = file
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| { 
            let splt: Vec<&str> = s.split(',').collect();
            return (Elf::from_str(splt[0]).unwrap(), Elf::from_str(splt[1]).unwrap())
        })
        .filter(|input| { match input {
            (left, right)
                if left.lower <= right.lower && left.upper >= right.upper => true,
            (left, right)
                if right.lower <= left.lower && right.upper >= left.upper => true,
            _ => false,
        }})
        .count();
    Ok(format!("{}", result))
}
