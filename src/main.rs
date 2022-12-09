#![feature(trait_alias)]
#![feature(downcast_unchecked)]
#![feature(box_into_inner)]
#![feature(let_chains)]

use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod consts;
mod fetcher;
mod solutions;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Usage: {:?} <day>", env::args().nth(0));
            process::exit(1)
        })
        .parse::<usize>()
        .unwrap_or_else(|_| {
            println!("err: Failed to parse day {:?}.", env::args().nth(1));
            process::exit(1)
        });

    if day >= solutions::DAYS {
        println!(
            "err: Solution day_{:02} not found in database (solutions.rs).",
            day
        );
        process::exit(1)
    }

    let input_file_str = format!("./input/day_{:02}.in", day);
    let input_file_path = Path::new(&input_file_str);

    if !input_file_path.is_file() {
        let fetcher = fetcher::fetch(day, consts::YEAR);
        let input = match fetcher {
            Err(e) => panic!("err: Fetcher returned error {}", e),
            Ok(input) => input,
        };
        if let Err(e) = fs::write(input_file_path, input) {
            panic!("err: Writing to file returned error {}", e);
        }
    } else {
        println!("{} exists, great!", input_file_str);
    }

    println!("Running solution for Day #{:02}!", day);

    let contents = fs::read_to_string(input_file_path).expect("file");
    let (part1, part2) = unsafe { solutions::solve(&contents, day) };
    println!("Part 1: {}, Part 2: {}", part1, part2);

    // Submit answers
    let env_submit = env::var("SUBMIT");
    if let Ok(env_submit) = env_submit {
        if env_submit.starts_with("1") {
            fetcher::submit(day, part1.to_string(), 1, 2022);
        } else if env_submit.starts_with("2") {
            fetcher::submit(day, part1.to_string(), 2, 2022);
        }
    }
}
