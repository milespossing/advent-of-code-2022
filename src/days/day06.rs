use std::collections::HashSet;

pub struct Solution {
    data: Vec<char>,
}

fn solve_with_width(width: usize, data: &Vec<char>) -> Option<usize> {
    let windows = data.windows(width);
    for (i, window) in windows.enumerate() {
        let set: HashSet<&char>  = std::collections::HashSet::from_iter(window);
        if set.len() == width { return Some(i + width) }
    }
    None
}

impl super::Solution for Solution {
    const DAY: u8 = 6;
    fn new(p: String) -> Self {
        let data: Vec<char> = std::fs::read_to_string(p).unwrap().chars().collect();
        Solution { data }
    }

    // I suppose that this would be easy enough to brute force..
    fn part1(&self) -> Result<String, String> {
        match solve_with_width(4, &self.data) {
            Some(v) => Ok(format!("{}", v)),
            None => Err(format!("None Found!"))
        }
    }
    fn part2(&self) -> Result<String, String> {
        match solve_with_width(14, &self.data) {
            Some(v) => Ok(format!("{}", v)),
            None => Err(format!("None Found!"))
        }
    }
}

