use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;

#[derive(Eq, Copy, Clone, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointErr;

impl FromStr for Point {
    type Err = ParsePointErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, s) = s.split_once(',').ok_or(ParsePointErr)?;
        let (y, z) = s.split_once(',').ok_or(ParsePointErr)?;
        Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
    }
}

static DIRS: [Point; 6] = [
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 0, z: -1 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
];

fn dfs(pt: Point, points: &HashSet<Point>, vis: &mut HashSet<Point>) {
    if pt.x < -5 || pt.y < -5 || pt.z < -5 || pt.x > 25 || pt.y > 25 || pt.z > 25 {
        return;
    }

    vis.insert(pt);
    for dir in DIRS {
        let new_pt = pt + dir;
        if !points.contains(&new_pt) && !vis.contains(&new_pt) {
            dfs(new_pt, points, vis);
        }
    }
}

pub fn solve(contents: &str) -> (usize, usize) {
    let points = contents
        .lines()
        .map(|c| c.parse::<Point>().unwrap())
        .collect::<HashSet<Point>>();
    let mut exterior = HashSet::new();

    let mut cnt1 = 0;
    let mut cnt2 = 0;

    // Fix potential recursion stack overflow
    stacker::grow(64 * 1024 * 1024, || {
        dfs(
            Point {
                x: -1,
                y: -1,
                z: -1,
            },
            &points,
            &mut exterior,
        );
    });

    for &pt in &points {
        for dir in DIRS {
            let new_pt = pt + dir;
            if !points.contains(&new_pt) {
                cnt1 += 1;

                if exterior.contains(&new_pt) {
                    cnt2 += 1;
                }
            }
        }
    }

    (cnt1, cnt2)
}
