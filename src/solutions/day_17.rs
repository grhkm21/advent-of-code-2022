use std::cmp::max;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

lazy_static! {
    static ref SHAPES: [Vec<(i64, i64)>; 5] = [
        [(0, 0), (1, 0), (2, 0), (3, 0)].into(),
        [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].into(),
        [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].into(),
        [(0, 0), (0, 1), (0, 2), (0, 3)].into(),
        [(0, 0), (0, 1), (1, 0), (1, 1)].into()
    ];
}

fn shift(pts: &[(i64, i64)], dx: i64, dy: i64) -> Vec<(i64, i64)> {
    let mut new_pts = pts.to_owned();
    for pt in new_pts.iter_mut() {
        *pt = (pt.0 + dx, pt.1 + dy);
    }
    new_pts.to_vec()
}

fn check(pts: &[(i64, i64)], rocks: &HashSet<(i64, i64)>) -> bool {
    for &key in pts {
        if key.0 < 0 || key.0 > 6 {
            return false;
        }
        if key.1 < 0 {
            return false;
        }
        if rocks.contains(&key) {
            return false;
        }
    }
    true
}

const N: i64 = 1000000000000;
pub fn solve(contents: &str) -> (usize, usize) {
    let instructions = contents
        .chars()
        .map(|c| if c == '>' { 1 } else { -1 })
        .collect::<Vec<i64>>();

    let mut period = 1;
    let mut period_inc = 0;

    let mut max_y = -1;
    let mut rocks = HashSet::new();

    let mut part1 = 0;

    // Place block by block, while keeping track of the top
    // 100 rows and see if it ever repeats.
    let mut k = 0;
    let mut ptr = 0;
    let mut height_map = HashMap::new();
    while k < N && (period == 1 || k % period != N % period) {
        // Base shape
        let mut cur_rock = shift(&SHAPES[(k % 5) as usize], 2, max_y + 4);
        k += 1;

        loop {
            // Blown by wind
            let dx = instructions[ptr];
            ptr = (ptr + 1) % instructions.len();

            let blown_rock = shift(&cur_rock, dx, 0);
            if check(&blown_rock, &rocks) {
                cur_rock = blown_rock;
            }

            // Drop by 1
            let dropped_rock = shift(&cur_rock, 0, -1);
            if !check(&dropped_rock, &rocks) {
                break;
            }
            cur_rock = dropped_rock;
        }

        for &rock in &cur_rock {
            rocks.insert(rock);
            max_y = max(max_y, rock.1);
        }

        if k == 2022 {
            part1 = max_y + 1;
        }

        // Extract top 70 rows
        if max_y >= 69 {
            let mut top_70_rows = Vec::new();
            for y in max_y - 69..=max_y {
                for x in 0..7 {
                    if rocks.contains(&(x, y)) {
                        top_70_rows.push((x, y - (max_y - 69)));
                    }
                }
            }
            // Found period
            if let Some((prev_k, prev_max_y)) = height_map.get(&top_70_rows) {
                period = k - prev_k;
                period_inc = max_y - prev_max_y;
            } else {
                height_map.insert(top_70_rows, (k, max_y));
            }
        }
    }

    let part2 = max_y + 1 + (N - k) / period * period_inc;

    (part1 as usize, part2 as usize)
}
