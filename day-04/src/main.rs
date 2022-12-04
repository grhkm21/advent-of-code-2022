use std::fs;
use std::mem;

const INT_ERR: &str = "err: can't parse int";
const FILE_ERR: &str = "err: can't read file";
const SPLIT_ERR: &str = "err: splitting failed";

struct Interval {
    l: usize,
    r: usize,
}

fn make_interval(s: &str) -> Interval {
    let parts = s.split_once("-").expect(SPLIT_ERR);
    Interval {
        l: parts.0.parse().expect(INT_ERR),
        r: parts.1.parse().expect(INT_ERR),
    }
}

fn cover_entire(x: &mut Interval, y: &mut Interval) -> bool {
    // [1, 4] -> [1, 2], [2, 3] but not [2, 5]
    if x.l > y.l {
        mem::swap(x, y);
    }
    (x.l == y.l) || (x.r >= y.r)
}

fn cover_partly(x: &mut Interval, y: &mut Interval) -> bool {
    // [1, 5] -> [2, 6], [4, 5] but not [6, 7]
    if x.l > y.l {
        mem::swap(x, y);
    }
    x.r >= y.l
}

fn solve_part_1() {
    let mut cnt = 0;
    let content = fs::read_to_string("input").expect(FILE_ERR);
    for line in content.lines().map(|s| s.split_once(",").expect(SPLIT_ERR)) {
        let mut interval1 = make_interval(line.0);
        let mut interval2 = make_interval(line.1);
        cnt += cover_entire(&mut interval1, &mut interval2) as usize;
    }
    println!("Part 1: {cnt}");
}

fn solve_part_2() {
    let mut cnt = 0;
    let content = fs::read_to_string("input").expect(FILE_ERR);
    for line in content.lines().map(|s| s.split_once(",").expect(SPLIT_ERR)) {
        let mut interval1 = make_interval(line.0);
        let mut interval2 = make_interval(line.1);
        cnt += cover_partly(&mut interval1, &mut interval2) as usize;
    }
    println!("Part 2: {cnt}");
}

fn main() {
    solve_part_1();
    solve_part_2();
}
