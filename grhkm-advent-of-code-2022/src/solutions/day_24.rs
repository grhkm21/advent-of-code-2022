use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::*;

fn round(grid: &Vec<Vec<Vec<char>>>, rd: i64) -> Vec<Vec<Vec<char>>> {
    let row = grid.len();
    let col = grid[0].len();
    let mut new_grid = vec![vec![vec![]; col]; row];

    let row = row as i64;
    let col = col as i64;

    // Simulate the movement and return a new grid
    for r in 0..row {
        for c in 0..col {
            for &chr in &grid[r as usize][c as usize] {
                let pos = Point::new(r, c);
                let mut new_pos = pos
                    + match chr {
                        '#' => Point::new(0, 0),
                        '^' => Point::new(-rd, 0),
                        'v' => Point::new(rd, 0),
                        '<' => Point::new(0, -rd),
                        '>' => Point::new(0, rd),
                        _ => unreachable!(),
                    };

                // Wrap around
                if chr != '#' {
                    let nrow = row - 2;
                    let ncol = col - 2;
                    new_pos.x = (new_pos.x % nrow - 1 + nrow) % nrow + 1;
                    new_pos.y = (new_pos.y % ncol - 1 + ncol) % ncol + 1;
                }
                new_grid[new_pos.x as usize][new_pos.y as usize].push(chr);
            }
        }
    }

    new_grid
}

pub fn solve(contents: &str) -> (usize, usize) {
    // We store a list of characters in each of grid[i][j]
    let grid = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '.' { vec![] } else { vec![c] })
                .collect()
        })
        .collect::<Vec<Vec<Vec<char>>>>();
    let row = grid.len();
    let col = grid[0].len();

    // Grid repeats every `cycles` iterations
    let cycles = lcm(row as i64, col as i64) as usize;
    let grids = (0..=cycles)
        .into_par_iter()
        .map(|rd| round(&grid, rd as i64))
        .collect::<Vec<_>>();

    let graph: HashMap<(usize, Point), Vec<(usize, Point)>> = (0..cycles)
        .into_par_iter()
        .map(|rd| {
            // Make edges from rd to rd + 1
            let cur_grid = &grids[rd];
            let nxt_grid = &grids[rd + 1];

            let mut graph = HashMap::new();
            let mut insert_edge = |x: Point, y: Point| {
                graph
                    .entry((rd, x))
                    .or_insert(Vec::new())
                    .push(((rd + 1) % cycles, y));
            };

            for r in 0..row {
                for c in 0..col {
                    let pt1 = Point::new(r as i64, c as i64);
                    if !cur_grid[r][c].is_empty() {
                        continue;
                    }
                    if nxt_grid[r][c].is_empty() {
                        insert_edge(pt1, pt1);
                    }
                    for pt2 in pt1.dirs4() {
                        if pt2.is_valid(row as i64, col as i64) && nxt_grid[pt2].is_empty() {
                            insert_edge(pt1, pt2);
                        }
                    }
                }
            }
            graph
        })
        .flatten()
        .collect();

    // BFS for shortest path
    let mut part1 = usize::MAX;
    let mut part2 = usize::MAX;

    let src = Point::new(0, 1);
    let dst = Point::new(row as i64 - 1, col as i64 - 2);

    let mut vis = HashSet::new();
    let mut deque = VecDeque::new();

    deque.push_back(((0, src), 0, 0));
    vis.insert((0, src, 0));
    while let Some(((rd, cur), dist, checkpoints)) = deque.pop_front() {
        // Check checkpoints
        let mut new_checkpoints = checkpoints;
        if cur == dst && new_checkpoints % 2 == 0 {
            new_checkpoints += 1;
            if new_checkpoints == 1 && dist < part1 {
                part1 = dist;
            }
            if new_checkpoints == 3 {
                part2 = dist;
                break;
            }
        } else if cur == src && new_checkpoints % 2 == 1 {
            new_checkpoints += 1;
        }

        // Find next step
        if !graph.contains_key(&(rd, cur)) {
            continue;
        }
        for (new_rd, dest) in &graph[&(rd, cur)] {
            let new_rd = *new_rd;
            let dest = *dest;

            if !vis.contains(&(new_rd, dest, new_checkpoints)) {
                vis.insert((new_rd, dest, new_checkpoints));
                deque.push_back(((new_rd, dest), dist + 1, new_checkpoints));
            }
        }
    }

    (part1, part2)
}
