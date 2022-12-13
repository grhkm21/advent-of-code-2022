use std::collections::VecDeque;

fn flood_fill(
    r: usize,
    c: usize,
    queue: &mut VecDeque<(usize, usize, usize)>,
    dist: &mut [Vec<usize>],
    grid: &mut [Vec<usize>],
) {
    while let Some((x, y, dt)) = queue.pop_front() {
        dist[x][y] = dt;

        let x = x as i32;
        let y = y as i32;
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if nx < 0 || nx >= r as i32 || ny < 0 || ny >= c as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[nx][ny] > grid[x as usize][y as usize] + 1 {
                continue;
            }

            if dt + 1 < dist[nx][ny] {
                dist[nx][ny] = dt + 1;
                queue.push_back((nx, ny, dt + 1));
            }
        }
    }
}

pub fn solve(contents: &str) -> (usize, usize) {
    let get_val = |c: char| {
        if c == 'S' {
            100
        } else if c == 'E' {
            120
        } else {
            (c as i8) - ('a' as i8)
        }
    };

    let mut grid = contents
        .lines()
        .map(|l| l.chars().map(|c| get_val(c).try_into().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();
    let r = grid.len();
    let c = grid[0].len();

    let mut sx = 0;
    let mut sy = 0;
    let mut ex = 0;
    let mut ey = 0;

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, g) in row.iter_mut().enumerate() {
            if *g == 100 {
                sx = i;
                sy = j;
                *g = 0;
            }
            if *g == 120 {
                ex = i;
                ey = j;
                *g = 25;
            }
        }
    }

    let mut dist = vec![vec![usize::MAX; c]; r];
    let mut queue = VecDeque::new();

    queue.push_back((sx, sy, 0));
    flood_fill(r, c, &mut queue, &mut dist, &mut grid);

    let part1 = dist[ex][ey];

    let mut dist = vec![vec![usize::MAX; c]; r];
    let mut queue = VecDeque::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &g) in row.iter().enumerate() {
            if g == 0 {
                queue.push_back((i, j, 0));
            }
        }
    }
    flood_fill(r, c, &mut queue, &mut dist, &mut grid);

    let part2 = dist[ex][ey];

    (part1, part2)
}
