pub struct Solver {
    content: String,
}

pub trait Solvable {
    fn solve(self) -> (usize, usize);
}
