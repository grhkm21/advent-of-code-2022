use std::{collections::BTreeSet, mem::swap};

fn fill_blocks(obstacles: &BTreeSet<(usize, usize)>, sx: usize, sy: usize, max_y: usize) -> usize {
    let mut obstacles = obstacles.clone();
    let mut cnt = 0;
    loop {
        // create block at (sx, sy)
        if obstacles.contains(&(sx, sy)) {
            break;
        }

        let (mut cx, mut cy) = (sx, sy);
        loop {
            // fall into oblivion
            if cy > max_y {
                break;
            }

            // move block down in the order specified
            if !obstacles.contains(&(cx, cy + 1)) {
                (cx, cy) = (cx, cy + 1);
                continue;
            } else if !obstacles.contains(&(cx - 1, cy + 1)) {
                (cx, cy) = (cx - 1, cy + 1);
                continue;
            } else if !obstacles.contains(&(cx + 1, cy + 1)) {
                (cx, cy) = (cx + 1, cy + 1);
                continue;
            }
            break;
        }

        if cy > max_y {
            break;
        }

        // treat block as obstacle
        cnt += 1;
        obstacles.insert((cx, cy));
    }

    cnt
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut obstacles = BTreeSet::new();

    let mut max_y = 0;
    for line in contents.lines() {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        for i in 0..points.len() - 1 {
            let (x1, y1) = points[i].split_once(',').unwrap();
            let (x2, y2) = points[i + 1].split_once(',').unwrap();

            let mut x1 = x1.parse::<usize>().unwrap();
            let mut y1 = y1.parse::<usize>().unwrap();
            let mut x2 = x2.parse::<usize>().unwrap();
            let mut y2 = y2.parse::<usize>().unwrap();

            if y1 > max_y {
                max_y = y1;
            }
            if y2 > max_y {
                max_y = y2;
            }

            // Fill in the obstacles (rocks) in between
            assert!(x1 == x2 || y1 == y2);
            if x1 > x2 {
                swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }

            for i in x1..=x2 {
                for j in y1..=y2 {
                    obstacles.insert((i, j));
                }
            }
        }
    }

    let max_y = *obstacles.iter().map(|(_, y)| y).max().unwrap();
    let part1 = fill_blocks(&obstacles, 500, 0, max_y);

    // horizontal distance can't be more than vertical distance
    for i in 500 - (max_y + 2)..=500 + (max_y + 2) {
        obstacles.insert((i, max_y + 2));
    }
    let part2 = fill_blocks(&obstacles, 500, 0, max_y + 2);

    (part1, part2)
}
