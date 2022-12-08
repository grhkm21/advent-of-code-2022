// use itertools::Itertools;
// use std::fmt;

// TODO: solve it lol
// I solved in python for leaderboard placement
// -> didn't go well
pub fn solve(contents: &str) -> (usize, usize) {
    // let lines = contents.lines();

    (
        contents
            .to_string()
            .chars()
            .nth(0)
            .expect("")
            .to_string()
            .parse()
            .unwrap(),
        0,
    )
}
