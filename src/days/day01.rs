const FILE_PATH: &str = "./inputs/day01.txt";

fn calorie_sum(elf: &str) -> u64 {
    elf.split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .sum()
}

pub fn solve_01() -> Result<String, String> {
    let text: String = std::fs::read_to_string(std::path::Path::new(FILE_PATH)).unwrap();
    let mut calories: Vec<u64> = text.as_str().split("\n\n").map(|e| calorie_sum(e)).collect();
    calories.sort_by(|a, b| b.cmp(a));
    let top1 = calories[0];
    let top3: u64 = calories.iter().take(3).sum();

    Ok(format!("Part 1: {}, Part 2: {}", top1, top3))
}
