use itertools::Itertools;
use std::ops::Mul;

pub fn parse_board(contents: &str) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for line in contents.lines() {
        res.push(
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<usize>()
                        .expect(&format!("err: failed to parse {:?}", c))
                })
                .collect(),
        )
    }
    res
}

trait CanAny {
    fn any(self) -> bool;
}

trait CanProduct<T> {
    fn prod(&mut self) -> T;
}

impl CanAny for [bool; 4] {
    fn any(self) -> bool {
        self[0] || self[1] || self[2] || self[3]
    }
}

impl<T: Copy + Mul<Output = T>> CanProduct<T> for [T; 4] {
    fn prod(&mut self) -> T {
        self[0] * self[1] * self[2] * self[3]
    }
}

pub fn solve(contents: &str) -> (usize, usize) {
    let board: Vec<Vec<usize>> = parse_board(contents);
    let r = board.len();
    let c = board[0].len();

    let look_dir = |x: isize, y: isize, dx: isize, dy: isize| {
        let r = r as isize;
        let c = c as isize;
        let mut res = Vec::new();
        let (mut cx, mut cy) = (x, y);
        while 0 <= cx && cx < r && 0 <= cy && cy < c {
            res.push(board[cx as usize][cy as usize] as isize);
            cx += dx;
            cy += dy;
        }
        res
    };

    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let dirs = (0..r)
        .cartesian_product(0..c)
        .map(|(x, y)| DIRS.map(|(dx, dy)| look_dir(x as isize, y as isize, dx, dy)))
        .collect::<Vec<[Vec<_>; 4]>>();

    let part1 = dirs
        .iter()
        .map(|arr| {
            arr.clone()
                .map(|v| v[0] > *v[1..].iter().max().unwrap_or(&-20))
                .any()
        })
        .map(|v: bool| v as usize)
        .sum();

    // up down right left
    let part2 = dirs
        .iter()
        .map(|arr| {
            arr.clone()
                .map(|v| {
                    (v.len(), {
                        (1..v.len())
                            .filter(move |&i| v[0] <= v[i])
                            .collect::<Vec<_>>()
                    })
                })
                .map(|(len, v)| match v[..] {
                    [] => len - 1,
                    [x, ..] => x,
                })
                .prod()
        })
        .map(|v: usize| v as usize)
        .max()
        .unwrap();

    (part1, part2)
}
