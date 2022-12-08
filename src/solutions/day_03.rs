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

pub fn solve(contents: &str) -> (usize, usize) {
    let mut part1 = 0;
    for line in contents.lines() {
        let n = line.len();
        let chars1 = (&line[0..n / 2]).chars().collect();
        let chars2 = (&line[n / 2..n]).chars().collect();
        part1 += priority(intersect(chars1, chars2)[0]);
    }

    let mut part2 = 0;
    let lines = contents.split("\n").collect::<Vec<&str>>();

    for i in (0..lines.len()).step_by(3) {
        let chars1 = lines[i].chars().collect();
        let chars2 = lines[i + 1].chars().collect();
        let chars3 = lines[i + 2].chars().collect();
        part2 += priority(
            [chars1, chars2, chars3]
                .into_iter()
                .reduce(intersect)
                .expect("")[0],
        );
    }

    (part1, part2)
}
