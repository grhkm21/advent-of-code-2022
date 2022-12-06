struct Solver;

trait Solvable {
    fn solve_part_1(&self);
    fn solve_part_2(&self);
}

impl Solvable for Solver {
    fn solve_part_1(&self) {
        println!("Part 1: 1234");
    }

    fn solve_part_2(&self) {
        println!("Part 2: wee");
    }
}

fn main() {
    let solver = Solver {};
    solver.solve_part_1();
    solver.solve_part_2();
}
