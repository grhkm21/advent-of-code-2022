pub fn solve(contents: &str) -> (usize, usize) {
    (contents.chars().nth(0).expect("") as usize, 1)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "input";

    #[test]
    fn test_day0() {
        let (part1, part2) = super::solve(INPUT);

        assert_eq!(part1, 0);
        assert_eq!(part2, 1);
    }
}
