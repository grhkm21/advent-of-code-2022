use itertools::Itertools;

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
        .map(|(x, y)| DIRS.map(|(dx, dy)| look_dir(x as isize, y as isize, dx, dy)));

    let mut part1 = 0;
    let mut part2 = 0;
    for views in dirs.clone() {
        let mut viewable = false;
        let mut view_prod = 1;
        for dir_vals in views {
            let mut cur_viewable = true;
            let len = dir_vals.len();
            let mut dist = None;
            for i in 1..len {
                // compute if dir_vals is viewable from the sidee
                // and the view distance, which can be checked by
                // checking whether dir_vals[0] is strictly greater than heights
                if dir_vals[0] <= dir_vals[i] {
                    cur_viewable = false;
                    dist = Some(dist.unwrap_or(i));
                }
            }
            if cur_viewable {
                viewable |= cur_viewable;
            }
            view_prod *= dist.unwrap_or(len - 1);
        }
        if viewable {
            part1 += 1;
        }
        part2 = part2.max(view_prod);
    }

    (part1, part2)
}
