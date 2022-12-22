#![feature(trait_alias)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]
#![feature(let_chains)]
#![feature(map_try_insert)]

use clap::{arg, command, ArgAction};
use std::fs;
use std::path::Path;
use std::process;

pub mod consts;
pub mod fetcher;
pub mod solutions;
pub mod solver;

struct Data {
    cookie_file_path: String,
    submit1: bool,
    submit2: bool
}

fn run_day_solution(day: usize, input_file_str: Option<&String>, data: &Data) {
    if day > solver::DAYS {
        process::exit(1)
    }

    let input_file_str = if let Some(input_file_str) = input_file_str {
        input_file_str.clone()
    } else {
        format!("./input/day_{day:02}.in")
    };
    let input_file_path = Path::new(&input_file_str);

    if !input_file_path.is_file() {
        let fetcher = fetcher::fetch(day, consts::YEAR, &data.cookie_file_path);
        let input = match fetcher {
            Err(e) => panic!("err: Fetcher returned error {e}"),
            Ok(input) => input,
        };
        if let Err(e) = fs::write(input_file_path, input) {
            panic!("err: Writing to file returned error {e}");
        }
    } else {
        println!("{input_file_path:?} exists, great!");
    }

    println!("Running solution with {input_file_path:?} for Day #{day:02}!");

    let contents = fs::read_to_string(input_file_path).unwrap();
    let contents = contents.trim();
    let (part1, part2) = solver::solve(contents, day);
    println!("Part 1: {part1}, Part 2: {part2}");

    if data.submit1 {
        fetcher::submit(day, part1, 1, consts::YEAR, &data.cookie_file_path);
    }

    if data.submit2 {
        fetcher::submit(day, part2, 2, consts::YEAR, &data.cookie_file_path);
    }
}

fn main() {
    let matches = command!()
        .arg(arg!(-d --day <DAY> "The day to run the solutions for").required(true))
        .arg(arg!(-c --cookie <COOKIE_FILE> "The cookie file to use").required(true))
        .arg(arg!(-i --input <INPUT_FILE> "The input file to use").required(false))
        .arg(
            arg!(--s1 ... "Include argument to submit part 1 of solution")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(--s2 ... "Include argument to submit part 2 of solution")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(--submit1 ... "Include argument to submit part 1 of solution")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(--submit2 ... "Include argument to submit part 2 of solution")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let day: String = matches.get_one::<String>("day").unwrap().clone();
    let cookie_file_path: String = matches.get_one::<String>("cookie").unwrap().clone();
    let input_file_path: Option<&String> = matches.get_one("input");
    let submit1 = matches.get_flag("submit1") || matches.get_flag("s1");
    let submit2 = matches.get_flag("submit2") || matches.get_flag("s2");
    
    let data = Data {
        cookie_file_path, submit1, submit2
    };

    if &day == "all" {
        if submit1 || submit2 {
            println!("warn: Submitting solutions is not supported with day = \"all\"");
        }
        for day in 1..=solver::DAYS {
            run_day_solution(day, input_file_path, &data);
        }
    } else {
        let day = day.parse::<usize>().unwrap_or_else(|_| {
            println!("err: Failed to parse day {day:?}");
            process::exit(1)
        });
        run_day_solution(day, input_file_path, &data);
    }
}
