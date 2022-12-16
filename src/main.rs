#![feature(trait_alias)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]
#![feature(let_chains)]
#![feature(map_try_insert)]

use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub mod consts;
pub mod fetcher;
pub mod solutions;
pub mod solver;

fn run_day_solution(day: usize, data_src: &str) {
    if day > solver::DAYS {
        println!("err: Solution day_{day:02} not found in database (solutions.rs).",);
        process::exit(1)
    }

    let input_file_str = format!("./input/{data_src}/day_{day:02}.in");
    let input_file_path = Path::new(&input_file_str);

    if !input_file_path.is_file() {
        if data_src == "sample" {
            panic!("Sample file {input_file_str} not found.");
        }
        let fetcher = fetcher::fetch(day, consts::YEAR);
        let input = match fetcher {
            Err(e) => panic!("err: Fetcher returned error {e}"),
            Ok(input) => input,
        };
        if let Err(e) = fs::write(input_file_path, input) {
            panic!("err: Writing to file returned error {e}");
        }
    } else {
        println!("{input_file_str} exists, great!");
    }

    println!("Running solution with {data_src} for Day #{day:02}!");

    let contents = fs::read_to_string(input_file_path).unwrap();
    let (part1, part2) = solver::solve(&contents, day);
    println!("Part 1: {part1}, Part 2: {part2}");

    // Submit answers
    let env_submit = env::var("SUBMIT");
    if let Ok(env_submit) = env_submit {
        if env_submit.starts_with('1') {
            fetcher::submit(day, part1, 1, consts::YEAR);
        } else if env_submit.starts_with('2') {
            fetcher::submit(day, part2, 2, consts::YEAR);
        }
    }
}

fn main() {
    let day_str = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: {:?} <day> [data/sample]", env::args().next());
        process::exit(1)
    });

    let data_src = if env::args().nth(2) == Some("sample".to_string()) {
        "sample"
    } else {
        "data"
    };

    if day_str == "all" {
        for day in 1..=solver::DAYS {
            run_day_solution(day, "data");
        }
    } else {
        let day = day_str.parse::<usize>().unwrap_or_else(|_| {
            println!("err: Failed to parse day {:?}.", env::args().nth(1));
            process::exit(1)
        });
        run_day_solution(day, data_src);
    }
}
