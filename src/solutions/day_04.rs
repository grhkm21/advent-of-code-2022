use std::mem;

const INT_ERR: &str = "err: can't parse int";
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

pub fn solve(contents: &str) -> (usize, usize) {
    let mut cnt1 = 0;
    let mut cnt2 = 0;

    for line in contents
        .lines()
        .map(|s| s.split_once(",").expect(SPLIT_ERR))
    {
        let mut interval1 = make_interval(line.0);
        let mut interval2 = make_interval(line.1);
        cnt1 += cover_entire(&mut interval1, &mut interval2) as usize;
        cnt2 += cover_partly(&mut interval1, &mut interval2) as usize;
    }

    (cnt1, cnt2)
}
