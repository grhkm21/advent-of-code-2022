struct Solver {
    &str content;
}

trait Solvable {
    fn solve(content: &str) -> (usize, usize);
}

impl Solvable for Solver {
    fn solve(content: &str) -> (usize, usize) {

    }
}

fn main() {
    let solver = Solver {};
    solver.solve_part_1();
    solver.solve_part_2();
}
