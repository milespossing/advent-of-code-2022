fn find_same(left: &[u8], right: &[u8]) -> Option<u8> {
    if left.len() == 0 || right.len() == 0 {
        return None
    }
    if left[0] == right[0] {
        return Some(left[0])
    }
    if left[0] > right[0] {
        return find_same(left, &right[1..]);
    } else {
        return find_same(&left[1..], right);
    }
}

fn to_id(c: u8) -> u8{
    match c {
        c if c > 96 => c - ('a' as u8) + 1,
        _ => c - ('A' as u8) + 27,
    }
}


fn find_value(s: &str) -> u64 {
    let len = s.len();
    let mut left = s[..len/2].as_bytes().iter().map(|&x| { to_id(x) }).collect::<Vec<u8>>();
    let mut right = s[len/2..].as_bytes().iter().map(|&x| { to_id(x) }).collect::<Vec<u8>>();
    left.sort();
    right.sort();
    match find_same(left.as_slice(), right.as_slice()) {
        Some(i) => i as u64,
        None => panic!("No matches")
    }
}

pub fn solve(p: &std::path::Path) -> Result<String, &str> {
    let file = std::fs::read_to_string(p.join("day03.txt")).unwrap();
    let lines = file.split('\n')
        .filter(|&s| !s.is_empty())
        .collect::<Vec<&str>>();
    let sum: u64 = lines.iter().map(|l| { find_value(l) }).sum();
    Ok(format!("{}", sum))
}
