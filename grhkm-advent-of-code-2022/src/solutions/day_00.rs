pub fn solve(contents: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
        let x = line.parse::<usize>().unwrap();
        part1 += x;
        part2 += x * 10;
    }

    (part1, part2)
}
