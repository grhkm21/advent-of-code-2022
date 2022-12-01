use std::fs;

fn solve_part_1() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("err: can't read file");

    // process each player separated by two new lines
    // keeping track of the current maximum sum
    let mut max_val = 0;
    for player in contents.split("\n\n") {
        let mut cur_val = 0;
        for line in player.split("\n") {
            let line_val = match line.parse::<i32>() {
                Ok(n) => n,
                Err(_e) => {
                    // println!("Error: {e}");
                    0
                }
            };
            cur_val += line_val;
        }
        
        // update maximum
        // println!("player: {cur_val}");
        if max_val < cur_val {
            max_val = cur_val;
        }
    }

    println!("Part 1: {max_val}");
}

fn solve_part_2() {
    let file_path = "input";
    let contents = fs::read_to_string(file_path).expect("err: can't read file");

    // process each player separated by two new lines
    // we store all values in a vector
    let mut vals = Vec::new();
    for player in contents.split("\n\n") {
        let mut cur_val = 0;

        // process each line of the player
        for line in player.split("\n") {
            let line_val = match line.parse::<i32>() {
                Ok(n) => n,
                Err(_e) => {
                    // println!("Error: {e}");
                    0
                }
            };
            cur_val += line_val;
        }

        // stores the total value of a player
        vals.push(cur_val);
    }

    // now compute sum of maximum 3 calories
    // sort_unstable_by allows custom comparator
    // which must return an `Ordering`
    vals.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let max_3_vals = vals[0] + vals[1] + vals[2];

    println!("Part 2: {max_3_vals}");
}

fn main() {
    solve_part_1();
    solve_part_2();
}