use std::fs;

fn round(move1: usize, move2: usize) -> usize {
    // returns score based on whether move2 beats move1
    return match [move1, move2] {
        [1, 1] => 3,
        [1, 2] => 6,
        [1, 3] => 0,
        [2, 1] => 0,
        [2, 2] => 3,
        [2, 3] => 6,
        [3, 1] => 6,
        [3, 2] => 0,
        [3, 3] => 3,
        _ => unreachable!(),
    };
}

fn solve_part_1() {
    let file_path = "input";
    let mut score = 0;

    for line in fs::read_to_string(file_path)
        .expect("err: can't read from file")
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
    {
        // convert to numbers
        let move1 = match &line[..] {
            &["A", _] => 1,
            &["B", _] => 2,
            &["C", _] => 3,
            _ => unreachable!(),
        };

        let move2 = match &line[..] {
            &[_, "X"] => 1,
            &[_, "Y"] => 2,
            &[_, "Z"] => 3,
            _ => unreachable!(),
        };

        // first add shape selected
        score += move2;

        // next add outcome of the round
        score += round(move1, move2);
    }

    println!("Part 1: {score}");
}

fn solve_part_2() {
    let file_path = "input";
    let mut score = 0;

    for line in fs::read_to_string(file_path)
        .expect("err: can't read from file")
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
    {
        // convert to numbers
        let move1 = match &line[..] {
            &["A", _] => 1,
            &["B", _] => 2,
            &["C", _] => 3,
            _ => unreachable!(),
        };

        let target_round = match &line[..] {
            &[_, "X"] => 0,
            &[_, "Y"] => 3,
            &[_, "Z"] => 6,
            _ => unreachable!(),
        };

        // we check all moves and see if we get our required results
        for move2 in 1..4 {
            if round(move1, move2) == target_round {
                // first add shape selected
                score += move2;

                // next add outcome of the round
                score += round(move1, move2);
            }
        }
    }

    println!("Part 2: {score}");
}

fn main() {
    solve_part_1();
    solve_part_2();
}
