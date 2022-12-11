pub mod day_00;
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;

pub enum SolverType {
    Integer,
    String,
}

pub const DAYS: usize = 9;
pub const SOLS: [(*const (), SolverType); DAYS + 1] = [
    (day_00::solve as *const (), SolverType::Integer),
    (day_01::solve as *const (), SolverType::Integer),
    (day_02::solve as *const (), SolverType::Integer),
    (day_03::solve as *const (), SolverType::Integer),
    (day_04::solve as *const (), SolverType::Integer),
    (day_05::solve as *const (), SolverType::String),
    (day_06::solve as *const (), SolverType::Integer),
    (day_07::solve as *const (), SolverType::Integer),
    (day_08::solve as *const (), SolverType::Integer),
    (day_09::solve as *const (), SolverType::Integer),
];

pub fn solve(contents: &str, day: usize) -> (String, String) {
    let (solver, solver_type) = &SOLS[day];

    let (part1, part2): (String, String) = unsafe {
        match solver_type {
            SolverType::Integer => {
                let code = std::mem::transmute::<&*const (), &fn(&str) -> (usize, usize)>(solver);
                let (part1, part2) = code(contents);
                (part1.to_string(), part2.to_string())
            }
            SolverType::String => {
                let code = std::mem::transmute::<&*const (), &fn(&str) -> (String, String)>(solver);
                let (part1, part2) = code(contents);
                (part1, part2)
            }
        }
    };

    (part1, part2)
}
