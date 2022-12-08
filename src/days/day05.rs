use std::str::FromStr;
use regex::Regex;

enum Stack {
    Cons(String, Box<Stack>),
    None,
}

struct State {
    stacks: Vec<Stack>,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut lines = s.split('\n').collect::<Vec<&str>>();
        lines.reverse();
        // this is the number of stacks to use
        let count = lines[0].split("  ").count();
        let mut state: Vec<Stack> = Vec::new();
        fn build_stack(start: usize, lines: &[&str]) -> Stack {
            if lines.len() == 0 || lines[0].chars()[start] == '' {
                Stack::None
            }

            Stack::None
        }
        for i in 0..count {
            let current
            state.push(Stack::None);
        }

        //.last().unwrap();
        //let count = last.trim();
        Err(format!("Not implemented"))
    }
}

struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}


#[test]
fn can_parse_instruction() {
    let inst = Instruction::from_str("move 5 from 6 to 2").unwrap();
    assert_eq!(inst.count, 5);
    assert_eq!(inst.source, 5);
    assert_eq!(inst.dest, 1);
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(instr: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(instr).unwrap();
        let count = if let Some(c) = captures.get(1) {
            usize::from_str_radix(c.as_str(), 10).unwrap()
        } else {
            panic!("no count");
        };
        let source = if let Some(s) = captures.get(2) {
            usize::from_str_radix(s.as_str(), 10).unwrap() - 1
        } else {
            panic!("no source");
        };
        let dest = if let Some(d) = captures.get(3) {
            usize::from_str_radix(d.as_str(), 10).unwrap() - 1
        } else {
            panic!("no source");
        };
        Ok(Instruction { count, source, dest })
    }
}

pub fn solve(p: &std::path::Path) -> Result<String, &str> {
    let data = std::fs::read_to_string(p.join("day05.txt")).unwrap();
    println!("{}", data);
    Err("Not implemented yet!")
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn 
//}
