pub fn solve(contents: &str) -> (String, String) {
    let mut x_pos: Vec<isize> = vec![1];
    let mut prev = 1;

    for line in contents.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => x_pos.push(prev),
            ["addx", arg] => {
                x_pos.push(prev);
                prev += arg.parse::<isize>().unwrap();
                x_pos.push(prev);
            }
            _ => unreachable!(),
        }
    }

    let part1: usize = (20..221)
        .step_by(40)
        .map(|i| i * x_pos[i - 1] as usize)
        .sum();

    let mut part2: String = "\n".to_owned();
    for y in 0..6 {
        for x in 0..40 {
            if (x as isize - x_pos[y * 40 + x]).abs() <= 1 {
                part2 += "#";
            } else {
                part2 += ".";
            }
        }
        part2 += "\n";
    }

    (part1.to_string(), part2.to_string())
}
