fn first_match(s: &str) -> Option<usize> {
    // return smallest index i such that
    // there exists j such that s[i] == s[j]
    let s: Vec<_> = s.chars().collect();
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if s[i] == s[j] {
                return Some(i);
            }
        }
    }
    None
}

fn solve(s: &str, k: usize) -> Option<usize> {
    // solves day-06, skipping forward to idx + 1 where idx is the first_match
    let n = s.len();
    let mut i = 0;
    while i < n {
        let start_pos = if i >= k - 1 { i + 1 - k } else { 0 };
        match first_match(&s[start_pos..i + 1]) {
            Some(idx) => {
                i += idx + 1;
            }
            None => {
                if i >= k - 1 {
                    return Some(i + 1);
                } else {
                    i += 1;
                }
            }
        }
    }
    None
}

fn main() {
    let s = include_str!("../input");
    println!("Part 1: {}", solve(s, 4).expect("err: can't find index"));
    println!("Part 2: {}", solve(s, 14).expect("err: can't find index"));
}
