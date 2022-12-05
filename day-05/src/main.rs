use std::fmt;
use std::fs;

use itertools::Itertools;

const FILE_ERR: &str = "err: can't read file";
const SPLIT_ERR: &str = "err: can't split string";
const TUPLE_ERR: &str = "err: can't unpack tuple";

// FIFO: stack, FILO: queue
#[derive(PartialEq)]
enum OperationOrder {
    FIFO,
    FILO,
}

impl fmt::Display for OperationOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OperationOrder::FIFO => write!(f, "Part 1 (FIFO): "),
            OperationOrder::FILO => write!(f, "Part 2 (FILO): "),
        }
    }
}

trait Applicable {
    fn apply<F, T2>(self, f: F) -> T2
    where
        F: Fn(Self) -> T2,
        Self: Sized,
    {
        f(self)
    }
}

impl<T> Applicable for T {}

fn solve(option: OperationOrder) {
    let contents = fs::read_to_string("input").expect(FILE_ERR);
    let (diagram, moves) = contents.split_once("\n\n").expect(SPLIT_ERR);
    let mut diagram_lines = diagram.lines().collect::<Vec<_>>();
    diagram_lines.reverse();

    // parse diagram into Vec<Vec<char>>
    let cols = 9;
    let mut rows = vec![Vec::<char>::new(); cols];

    for row in &diagram_lines[1..] {
        let row = row.chars().collect::<Vec<_>>();
        for i in 0..cols {
            let c = row[i * 4 + 1];
            if c != ' ' {
                rows[i].push(c);
            }
        }
    }

    // parse moves
    let pw = |x: &str| x.parse::<usize>().unwrap();
    let moves = moves.lines().map(|l| {
        l.split_whitespace()
            .next_tuple::<(_, _, _, _, _, _)>()
            .expect(TUPLE_ERR)
            .apply(|(_, x, _, y, _, z)| (pw(x), pw(y), pw(z)))
    });

    // process moves, creating temporary queue
    for (num, src_idx, dest_idx) in moves {
        // pop from [src] stack
        let src = &mut rows[src_idx - 1];
        let mut tmp = src[src.len() - num..].to_vec();

        if option == OperationOrder::FIFO {
            tmp.reverse();
        }

        src.truncate(src.len() - num);
        rows[dest_idx - 1].extend(tmp);
    }

    println!(
        "{option}{}",
        rows.into_iter().map(|r| r[r.len() - 1]).collect::<String>()
    );
}

fn main() {
    solve(OperationOrder::FIFO);
    solve(OperationOrder::FILO);
}
