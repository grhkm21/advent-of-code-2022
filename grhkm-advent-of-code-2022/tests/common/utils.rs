use advent_of_code_2022::solver;
use std::fs;

pub fn get(day: usize) -> Result<String, std::io::Error> {
    fs::read_to_string(format!("./input/sample/day_{day:02}.in"))
}

pub fn solve(day: usize) -> (String, String) {
    let sample = get(day);
    if sample.is_err() {
        panic!("Error: {}", sample.unwrap_err());
    }
    let sample = sample.unwrap();
    solver::solve(&sample, day)
}

macro_rules! solutions_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, (exp_part1, exp_part2)) = $value;
            let (sol_part1, sol_part2) = solve(input);

            assert_eq!(format!("{exp_part1}"), format!("{sol_part1}"));
            assert_eq!(format!("{exp_part2}"), format!("{sol_part2}"));
        }
    )*
    }
}
