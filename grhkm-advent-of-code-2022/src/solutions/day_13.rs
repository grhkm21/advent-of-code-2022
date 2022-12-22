use std::cmp::min;

fn parse_arr(s: &str) -> Vec<&str> {
    if s == "[]" {
        return Vec::new();
    }

    let s = &s[1..s.len()];
    let mut v = Vec::new();
    let mut prev_index = 0;
    let mut level = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            level += 1;
        }
        if c == ']' {
            level -= 1;
        }

        if (c == ',') && level == 0 {
            v.push(&s[prev_index..i]);
            prev_index = i + 1;
        }
    }
    v.push(&s[prev_index..s.len() - 1]);
    v
}

fn cmp_str(a: &str, b: &str) -> Option<bool> {
    if a.is_empty() && b.is_empty() {
        return None;
    } else if a.is_empty() {
        return Some(false);
    } else if b.is_empty() {
        return Some(true);
    }

    let is_list_a = a.starts_with('[');
    let is_list_b = b.starts_with('[');

    if !is_list_a && !is_list_b {
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        if a != b {
            return Some(a < b);
        }
        None
    } else if is_list_a && is_list_b {
        // extract data
        let list_a = parse_arr(a);
        let list_b = parse_arr(b);

        for i in 0..min(list_a.len(), list_b.len()) {
            let cmp = cmp_str(list_a[i], list_b[i]);
            if let Some(c) = cmp {
                return Some(c);
            }
        }

        if list_a.len() != list_b.len() {
            return Some(list_a.len() < list_b.len());
        }

        return None;
    } else if is_list_a {
        return cmp_str(a, &format!("[{b}]"));
    } else {
        return cmp_str(&format!("[{a}]"), b);
    }
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut part1 = 0;
    let groups = contents.split("\n\n").collect::<Vec<&str>>();
    for (i, group) in groups.iter().enumerate() {
        let (a, b) = group.split_once('\n').unwrap();
        let res = cmp_str(a, b);
        if res == Some(true) {
            part1 += i + 1;
        }
    }

    let mut groups = groups
        .iter()
        .flat_map(|c| c.split('\n').collect::<Vec<&str>>())
        .collect::<Vec<&str>>();

    groups.push("[[2]]");
    groups.push("[[6]]");

    let n = groups.len();
    for _ in 0..n - 1 {
        for j in 0..n - 1 {
            if cmp_str(groups[j], groups[j + 1]) == Some(false) {
                groups.swap(j, j + 1)
            }
        }
    }

    let mut part2 = 1;
    for (i, &g) in groups.iter().enumerate() {
        if g == "[[2]]" || g == "[[6]]" {
            part2 *= i + 1;
        }
    }

    (part1, part2)
}
