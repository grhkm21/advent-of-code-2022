use std::fs;

fn priority(c: char) -> usize {
    let c = c as u8;
    (match c {
        b'A'..=b'Z' => c - 0x41 + 1,
        b'a'..=b'z' => c - 0x61 + 1,
        _ => unreachable!(),
    }) as usize
}

fn intersect(s1: Vec<char>, s2: Vec<char>) -> Vec<char> {
    let mut res = Vec::new();
    for c in s1 {
        if s2.contains(&c) {
            res.push(c);
        }
    }
    res
}

// fn _intersect<'a>(s1: &'a str, s2: &'a str) -> impl Iterator<Item = char> + 'a {
//     s1.chars().filter(|c| s2.contains(&c.to_string()))
// }

fn solve_part_1() {
    let mut sum = 0;
    for line in fs::read_to_string("input").expect("").split("\n") {
        let n = line.len();
        let chars1 = (&line[0..n / 2]).chars().collect();
        let chars2 = (&line[n / 2..n]).chars().collect();
        sum += priority(intersect(chars1, chars2)[0]);
    }
    println!("Part 1: {sum}");
}

fn solve_part_2() {
    let mut sum = 0;
    let binding = fs::read_to_string("input").expect("");
    let lines = binding.split("\n").collect::<Vec<&str>>();

    for i in (0..lines.len()).step_by(3) {
        let chars1 = lines[i].chars().collect();
        let chars2 = lines[i + 1].chars().collect();
        let chars3 = lines[i + 2].chars().collect();
        sum += priority(
            [chars1, chars2, chars3]
                .into_iter()
                .reduce(intersect)
                .expect("")[0],
        );
    }
    println!("Part 2: {sum}");
}

fn main() {
    solve_part_1();
    solve_part_2();
}
