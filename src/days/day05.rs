use regex::Regex;
use std::str::FromStr;

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

struct State {
    stacks: Vec<Stack<char>>,
}

impl State {
    pub fn pop(&mut self, i: usize) -> Option<char> {
        if i >= self.stacks.len() {
            return None;
        }
        let stack = &mut self.stacks[i];
        stack.pop()
    }
    
    pub fn push(&mut self, i: usize, c: char) {
        if i >= self.stacks.len() {
            panic!("Stack index out of range");
        }
        self.stacks[i].push(c);
    }

}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut lines = s
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        lines.reverse();
        let headers = lines[0]
            .iter()
            .enumerate()
            .filter(|&(_, c)| !c.is_whitespace());
        fn create_stack(lines: &[Vec<char>], index: usize) -> Stack<char> {
            let mut stack = Stack::<char>::new();
            for line in lines.iter() {
                if line[index].is_whitespace() {
                    return stack;
                }
                stack.push(line[index]);
            }
            return stack;
        }
        let stacks = headers.map(|(i, _)| create_stack(&lines[1..], i)).collect();
        Ok(State { stacks })
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

#[test]
fn can_execute_instruction() {
    let mut state = State { stacks: vec![Stack::<char>::new(), Stack::<char>::new()] };
    state.push(0, 'A');
    state.push(0, 'B');
    state.push(0, 'C');
    state.push(0, 'D');
    state.push(1, 'E');
    state.push(1, 'F');
    state.push(1, 'G');
    state.push(1, 'H');
    let instruction = Instruction::from_str("move 2 from 2 to 1").unwrap();
    instruction.execute(&mut state);
    assert_eq!(state.stacks[0].data, vec!('A', 'B', 'C', 'D', 'H', 'G'));
    assert_eq!(state.stacks[1].data, vec!('E', 'F'));
}

impl Instruction {
    pub fn execute(&self, state: &mut State) {
        for _ in 0..self.count {
            let c = state.pop(self.source).unwrap();
            state.push(self.dest, c);
        }
    }
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
        Ok(Instruction {
            count,
            source,
            dest,
        })
    }
}

pub fn solve(p: &std::path::Path) -> Result<String, &str> {
    let data = std::fs::read_to_string(p.join("day05.txt")).unwrap();
    let split = data.split("\n\n").collect::<Vec<&str>>();
    let mut state = State::from_str(split[0]).unwrap();
    let instructions = split[1]
        .split('\n')
        .filter(|s|!s.is_empty())
        .map(|s| Instruction::from_str(s).unwrap())
        .collect::<Vec<Instruction>>();
    for instruction in instructions.iter() {
        instruction.execute(&mut state);
    }
    let message = state.stacks.iter_mut().map(|s| s.pop().unwrap()).collect::<String>();
    Ok(message)
}
