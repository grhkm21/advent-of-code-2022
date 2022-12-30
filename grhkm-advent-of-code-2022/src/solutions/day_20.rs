#[derive(Copy, Clone, Debug)]
struct Item {
    val: i64,
    idx: usize,
}

fn mod_add(x: i64, y: i64, m: i64) -> i64 {
    let r = (x + y) % m;
    (r + m) % m
}

fn solve_arr(arr: &[Item], times: usize) -> i64 {
    let mut arr = arr.to_owned();
    let len = arr.len();
    for _ in 0..times {
        for i in 0..len {
            // Find original item
            let j = (0..len).find(|&k| arr[k].idx == i).unwrap();
            let item = arr[j];
            arr.remove(j);

            // Insert item at new position
            let new_j = mod_add(j as i64, item.val, (len - 1) as i64) as usize;
            arr.insert(new_j, item);
        }
    }

    // Find position of 0
    let pos = (0..len).find(|&k| arr[k].val == 0).unwrap();
    arr[(pos + 1000) % len].val + arr[(pos + 2000) % len].val + arr[(pos + 3000) % len].val
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut arr = contents
        .lines()
        .enumerate()
        .map(|(i, l)| Item {
            val: l.parse().unwrap(),
            idx: i,
        })
        .collect::<Vec<_>>();

    let part1 = solve_arr(&arr, 1) as usize;

    for item in arr.iter_mut() {
        *item = Item {
            val: item.val * 811589153,
            idx: item.idx,
        };
    }
    let part2 = solve_arr(&arr, 10) as usize;

    (part1, part2)
}
