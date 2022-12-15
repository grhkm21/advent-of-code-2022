use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};

const N: i64 = 4000000;
const TARGET_Y: i64 = 2000000;

fn dist(x: i64, y: i64, u: i64, v: i64) -> i64 {
    (u - x).abs() + (v - y).abs()
}

pub fn solve(contents: &str) -> (usize, usize) {
    lazy_static! {
        static ref PARSE_INT: Regex = Regex::new(r"-?\d+").unwrap();
    }

    let mut data = Vec::new();
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_dist = i64::MIN;

    for line in contents.lines() {
        let mut fields = PARSE_INT.captures_iter(line);
        let x = fields.next().unwrap()[0].parse().unwrap();
        let y = fields.next().unwrap()[0].parse().unwrap();
        let nx = fields.next().unwrap()[0].parse().unwrap();
        let ny = fields.next().unwrap()[0].parse().unwrap();
        min_x = min(min_x, min(x, nx));
        max_x = max(max_x, max(x, nx));
        max_dist = max(max_dist, dist(x, y, nx, ny));
        data.push((x, y, nx, ny));
    }

    let mut part1 = 0;
    for u in min_x - max_dist..=max_x + max_dist {
        let mut impossible = false;
        for (x, y, nx, ny) in &data {
            let (x, y, nx, ny) = (*x, *y, *nx, *ny);

            if (u, TARGET_Y) == (x, y) || (u, TARGET_Y) == (nx, ny) {
                // If it overlaps with existing beacon, skip following check
                impossible = false;
                continue;
            }

            if dist(x, y, u, TARGET_Y) <= dist(x, y, nx, ny) {
                impossible = true;
                break;
            }
        }
        if impossible {
            part1 += 1;
        }
    }

    for u in 0..=N {
        // For each data, calculate the range on y=v
        // where it's impossible to have another beacon
        let mut impossible = Vec::new();
        for (x, y, nx, ny) in &data {
            let (x, y, nx, ny) = (*x, *y, *nx, *ny);

            // Original distance
            let dt = dist(x, y, nx, ny);

            // Each step we move away from x
            // the impossible range shrinks by 1
            let new_dt = dt - (u - x).abs();
            if new_dt <= 0 {
                continue;
            }

            impossible.push((max(0, y - new_dt), min(N, y + new_dt)));
        }
        impossible.sort();

        // Sort by left endpoint and merge intervals
        fn intersect(l1: i64, r1: i64, l2: i64, r2: i64) -> Option<(i64, i64)> {
            if l1 > l2 {
                return intersect(l2, r2, l1, r1);
            }
            if r1 + 1 < l2 {
                return None;
            }
            Some((l1, max(r1, r2)))
        }

        let mut i = 0;
        while i < impossible.len() - 1 {
            let (l1, r1) = impossible[i];
            let (l2, r2) = impossible[i + 1];
            if let Some((l, r)) = intersect(l1, r1, l2, r2) {
                impossible[i] = (l, r);
                impossible.remove(i + 1);
            } else {
                i += 1;
            }
        }

        if impossible.len() > 1 {
            assert_eq!(impossible[0].1 + 1, impossible[1].0 - 1);
            let v = impossible[0].1 + 1;
            return (part1, (u * N + v).try_into().unwrap());
        }
    }

    panic!("Part 2 not found");
}
