use crate::utils::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn round(elves: &mut HashSet<Point>, round: usize) -> bool {
    let mut preferred: HashMap<Point, Point> = HashMap::new();
    let mut preferred_cnt: HashMap<Point, u64> = HashMap::new();

    for &elf in elves.iter() {
        // count neighbors
        let mut prefer = elf;
        if elf
            .dirs8()
            .iter()
            .filter(|&elf| elves.contains(elf))
            .count()
            > 0
        {
            for i in round..round + 4 {
                let dir = DIRS4[i % 4];
                let forward_ahead = elf + dir;
                let forward_left = forward_ahead + dir.left();
                let forward_right = forward_ahead + dir.right();
                if !elves.contains(&forward_ahead)
                    && !elves.contains(&forward_left)
                    && !elves.contains(&forward_right)
                {
                    prefer = elf + dir;
                    break;
                }
            }
        }

        preferred.insert(elf, prefer);
        preferred_cnt
            .entry(prefer)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    elves.clear();
    let mut accepted_prefer: bool = false;
    for (elf, prefer) in preferred.iter() {
        if elf == prefer || preferred_cnt[prefer] > 1 {
            elves.insert(*elf);
        } else {
            elves.insert(*prefer);
            accepted_prefer = true;
        }
    }
    accepted_prefer
}

fn get_bounding_box(grid: &HashSet<Point>) -> (i64, i64, i64, i64) {
    let mut minx = i64::MAX;
    let mut miny = i64::MAX;
    let mut maxx = i64::MIN;
    let mut maxy = i64::MIN;

    for pt in grid {
        minx = min(minx, pt.x);
        miny = min(miny, pt.y);
        maxx = max(maxx, pt.x);
        maxy = max(maxy, pt.y);
    }

    (minx, miny, maxx, maxy)
}

#[allow(dead_code)]
fn print_grid(grid: &HashSet<Point>) {
    // find bounding rectangle
    let (minx, miny, maxx, maxy) = get_bounding_box(grid);

    // print grid
    for x in minx..=maxx {
        for y in miny..=maxy {
            print!(
                "{}",
                if grid.contains(&Point::new(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!();
}

pub fn solve(contents: &str) -> (usize, usize) {
    let grid = contents
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // extract elf positions
    let mut elves: HashSet<Point> = HashSet::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, &chr) in row.iter().enumerate() {
            if chr == '#' {
                let x = x as i64;
                let y = y as i64;
                elves.insert(Point::new(x, y));
            }
        }
    }

    let mut part1 = 0;
    let mut r = 1;
    while round(&mut elves, r - 1) {
        if r == 10 {
            let (minx, miny, maxx, maxy) = get_bounding_box(&elves);
            part1 = ((maxx - minx + 1) * (maxy - miny + 1)) as usize - elves.len();
        }
        r += 1;
    }

    (part1, r)
}
