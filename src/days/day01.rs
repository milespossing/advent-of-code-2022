const FILE_PATH: &str = "./inputs/day01.txt";

fn calorie_sum(elf: &str) -> u64 {
    elf.split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .sum()
}

pub fn solve_01() -> Result<String, String> {
    let text: String = std::fs::read_to_string(std::path::Path::new(FILE_PATH)).unwrap();
    Ok(format!("{}", text.as_str().split("\n\n").map(|e| calorie_sum(e)).max().unwrap()))
}
