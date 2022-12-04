use std::str::FromStr;

#[derive(Copy, Clone)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    pub fn from_char(c: &str) -> Result<Play, String> {
        match c {
            "X" | "A" => Ok(Play::Rock),
            "Y" | "B" => Ok(Play::Paper),
            "Z" | "C" => Ok(Play::Scissors),
            _ => Err(format!("{} is not a proper play", c)),
        }
    }

    pub fn fight(&self, other: &Play) -> i32 {
        let result = match self {
            Play::Rock => match other {
                Play::Rock => 3,
                Play::Scissors => 6,
                Play::Paper => 0,
            }
            Play::Paper => match other {
                Play::Rock => 6,
                Play::Scissors => 0,
                Play::Paper => 3,
            }
            Play::Scissors => match other {
                Play::Rock => 0,
                Play::Scissors => 3,
                Play::Paper => 6,
            }
        };
        result + (*self as i32)
    }
}

struct Strategy {
    theirs: Play,
    mine: Play,
}


impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<&str> = s.split(' ').collect();
        Ok(Strategy { theirs: Play::from_char(chars[0]).unwrap(), mine: Play::from_char(chars[1]).unwrap() })
    }
}

impl Strategy {
    pub fn score(&self) -> i32 {
        return self.mine.fight(&self.theirs);
    }
}

pub fn solve(path: &std::path::Path) -> Result<String, String> {
    let lines = std::fs::read_to_string(path.join("day02.txt")).unwrap();
    let strat: Vec<Strategy> = lines.split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| { Strategy::from_str(s).unwrap() } ).collect();
    Ok(format!("{}", strat.iter().map(|s| { s.score() }).sum::<i32>()))
}
