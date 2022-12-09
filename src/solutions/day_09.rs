use std::collections::HashSet;

pub fn sgn(x: i32) -> i32 {
    if x > 0 {
        return 1;
    }
    if x < 0 {
        return -1;
    }
    0
}

pub fn get_dir(dir: &str) -> (i32, i32) {
    return match dir {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => unreachable!(),
    };
}

pub fn pull(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    // returns (dx, dy) indicating where tail should move
    let dx;
    let dy;

    // If close enough, don't move
    // Otherwise, pull in whichever direction changes
    if (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1 {
        dx = 0;
        dy = 0;
    } else {
        dx = sgn(head_x - tail_x);
        dy = sgn(head_y - tail_y);
    }
    // whichever direction changes, move that way
    (tail_x + dx, tail_y + dy)
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut head_x: i32 = 0;
    let mut head_y: i32 = 0;
    let mut tail_x: i32 = 0;
    let mut tail_y: i32 = 0;

    let mut vis: HashSet<(i32, i32)> = HashSet::new();
    vis.insert((0, 0));

    for line in contents.split("\n") {
        if let Some((dir, len)) = line.split_once(" ") {
            let len = len
                .parse::<isize>()
                .expect("err: Failed to parse int {len}");
            let (dx, dy) = get_dir(dir);
            for _ in 0..len {
                head_x += dx;
                head_y += dy;
                (tail_x, tail_y) = pull(head_x, head_y, tail_x, tail_y);
                vis.insert((tail_x, tail_y));
            }
        } else {
            panic!("err: Failed to read line {line}");
        }
    }

    let part1 = vis.len();

    (part1, 0)
}
