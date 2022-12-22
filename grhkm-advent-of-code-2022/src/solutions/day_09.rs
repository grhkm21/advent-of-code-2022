use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, pt: Pos) -> Pos {
        Pos::new(pt.x + self.x, pt.y + self.y)
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, pt: Pos) -> Pos {
        Pos::new(pt.x - self.x, pt.y - self.y)
    }
}

fn sgn(x: i32) -> i32 {
    if x > 0 {
        return 1;
    }
    if x < 0 {
        return -1;
    }
    0
}

fn get_dir(dir: &str) -> Pos {
    match dir {
        "R" => Pos::new(1, 0),
        "L" => Pos::new(-1, 0),
        "U" => Pos::new(0, 1),
        "D" => Pos::new(0, -1),
        _ => unreachable!(),
    }
}

fn pull(head: Pos, tail: Pos) -> Pos {
    // If close enough, don't move
    // Otherwise, pull in whichever direction changes
    let mut dir = tail - head;
    if dir.x.abs() <= 1 && dir.y.abs() <= 1 {
        dir = Pos::new(0, 0);
    } else {
        // whichever direction changes, move that way
        dir.x = sgn(dir.x);
        dir.y = sgn(dir.y);
    }
    tail + dir
}

fn simulate(chains: usize, contents: &str) -> usize {
    let mut vis: HashSet<Pos> = HashSet::new();
    let mut chain_pos: Vec<Pos> = vec![Pos::new(0, 0); chains];

    vis.insert(chain_pos[chains - 1]);
    for line in contents.split('\n') {
        if let Some((dir, len)) = line.split_once(' ') {
            let len = len
                .parse::<isize>()
                .expect("err: Failed to parse int {len}");
            for _ in 0..len {
                chain_pos[0] = chain_pos[0] + get_dir(dir);
                for i in 1..chains {
                    chain_pos[i] = pull(chain_pos[i - 1], chain_pos[i]);
                }
                vis.insert(chain_pos[chains - 1]);
            }
        } else {
            panic!("err: Failed to read line {line}");
        }
    }

    vis.len()
}

pub fn solve(contents: &str) -> (usize, usize) {
    let part1 = simulate(2, contents);
    let part2 = simulate(10, contents);

    (part1, part2)
}
