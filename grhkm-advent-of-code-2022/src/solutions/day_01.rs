pub fn solve(contents: &str) -> (usize, usize) {
    // process each player separated by two new lines
    // keeping track of the current maximum sum
    let mut max_val = 0;
    let mut vals = Vec::new();
    for player in contents.split("\n\n") {
        let mut cur_val = 0;
        for line in player.split('\n') {
            let line_val = match line.parse::<usize>() {
                Ok(n) => n,
                Err(_e) => {
                    // println!("Error: {e}");
                    0
                }
            };
            cur_val += line_val;
            vals.push(cur_val);
        }

        // update maximum
        // println!("player: {cur_val}");
        if max_val < cur_val {
            max_val = cur_val;
        }
    }

    // now compute sum of maximum 3 calories
    // sort_unstable_by allows custom comparator
    // which must return an `Ordering`
    vals.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let max_3_vals = vals[0] + vals[1] + vals[2];

    (max_val, max_3_vals)
}
