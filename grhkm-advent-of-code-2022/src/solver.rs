use std::time::Instant;

use crate::solutions::*;

#[allow(dead_code)]
pub enum SolverType {
    BigInteger,
    Integer,
    String,
}

pub const DAYS: usize = 25;
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
    (day_10::solve as *const (), SolverType::String),
    (day_11::solve as *const (), SolverType::Integer),
    (day_12::solve as *const (), SolverType::Integer),
    (day_13::solve as *const (), SolverType::Integer),
    (day_14::solve as *const (), SolverType::Integer),
    (day_15::solve as *const (), SolverType::Integer),
    (day_16::solve as *const (), SolverType::Integer),
    (day_17::solve as *const (), SolverType::Integer),
    (day_18::solve as *const (), SolverType::Integer),
    (day_19::solve as *const (), SolverType::Integer),
    (day_20::solve as *const (), SolverType::Integer),
    (day_21::solve as *const (), SolverType::Integer),
    (day_22::solve as *const (), SolverType::Integer),
    (day_23::solve as *const (), SolverType::Integer),
    (day_24::solve as *const (), SolverType::Integer),
    (day_25::solve as *const (), SolverType::String),
];

pub fn solve(contents: &str, day: usize) -> (String, String) {
    let (solver, solver_type) = &SOLS[day];

    let now = Instant::now();
    let (part1, part2): (String, String) = unsafe {
        match solver_type {
            SolverType::BigInteger => {
                use num_bigint::BigUint;
                let code =
                    std::mem::transmute::<&*const (), &fn(&str) -> (BigUint, BigUint)>(solver);
                let (part1, part2) = code(contents);
                (part1.to_string(), part2.to_string())
            }
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
    println!("Day {day:02} time taken: {}ms", now.elapsed().as_millis());
    (part1, part2)
}
