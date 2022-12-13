fn round(move1: usize, move2: usize) -> usize {
    // returns score based on whether move2 beats move1
    match [move1, move2] {
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
    }
}

fn parse_tuples(from: &str) -> Vec<(&str, &str)> {
    from.trim()
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .collect()
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in parse_tuples(contents) {
        // convert to numbers
        let move1 = match &line {
            ("A", _) => 1,
            ("B", _) => 2,
            ("C", _) => 3,
            _ => unreachable!(),
        };

        let move2 = match &line {
            (_, "X") => 1,
            (_, "Y") => 2,
            (_, "Z") => 3,
            _ => unreachable!(),
        };

        let target_round = match &line {
            (_, "X") => 0,
            (_, "Y") => 3,
            (_, "Z") => 6,
            _ => unreachable!(),
        };

        // part 1: first add shape selected
        part1 += move2;

        // part 1: next add outcome of the round
        part1 += round(move1, move2);

        // part 2: we check all moves and see if we get our required results
        for move2 in 1..4 {
            if round(move1, move2) == target_round {
                // first add shape selected
                part2 += move2;

                // next add outcome of the round
                part2 += round(move1, move2);

                break;
            }
        }
    }

    (part1, part2)
}
