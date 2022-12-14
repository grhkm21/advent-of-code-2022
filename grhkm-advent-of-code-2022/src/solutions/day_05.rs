use std::fmt;

use itertools::Itertools;

const SPLIT_ERR: &str = "err: can't split string";
const TUPLE_ERR: &str = "err: can't unpack tuple";

// Fifo: stack, Filo: queue
#[derive(PartialEq)]
enum OperationOrder {
    Fifo,
    Filo,
}

impl fmt::Display for OperationOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OperationOrder::Fifo => write!(f, "Part 1 (Fifo): "),
            OperationOrder::Filo => write!(f, "Part 2 (Filo): "),
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

fn _solve(contents: &str, option: OperationOrder) -> String {
    let (diagram, moves) = contents.split_once("\n\n").expect(SPLIT_ERR);
    let mut diagram_lines = diagram.lines().collect::<Vec<_>>();
    diagram_lines.reverse();

    // parse diagram into Vec<Vec<char>>
    let cols = diagram_lines[0].split_whitespace().count();
    let mut rows = vec![Vec::new(); cols];

    for row in &diagram_lines[1..] {
        let row = row.chars().collect::<Vec<_>>();
        println!("{row:?}");
        for i in 0..cols {
            let c = row[i * 4 + 1];
            if c != ' ' {
                rows[i].push(c);
                print!("{i} -> {c}");
            }
        }
        println!();
    }
    println!("rows: {rows:?}");

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

        if option == OperationOrder::Fifo {
            tmp.reverse();
        }

        src.truncate(src.len() - num);
        rows[dest_idx - 1].extend(tmp);
    }

    rows.into_iter().map(|r| r[r.len() - 1]).collect::<String>()
}

pub fn solve(contents: &str) -> (String, String) {
    let part1 = _solve(contents, OperationOrder::Fifo);
    let part2 = _solve(contents, OperationOrder::Filo);
    (part1, part2)
}
