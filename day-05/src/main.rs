use std::collections::VecDeque;
use std::fmt;
use std::fs;

const INT_ERR: &str = "err: can't parse int";
const POP_ERR: &str = "err: can't pop from data structure";
const FILE_ERR: &str = "err: can't read file";
const SPLIT_ERR: &str = "err: can't split string";

// FIFO: stack, FILO: queue
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

fn solve(option: OperationOrder) {
    let contents = fs::read_to_string("input").expect(FILE_ERR);
    let (diagram, moves) = contents.split_once("\n\n").expect(SPLIT_ERR);
    let mut diagram_lines = diagram.split("\n").collect::<Vec<&str>>();
    diagram_lines.reverse();

    // parse diagram into Vec<VecDeque<char>>
    let cols = 9;
    let mut rows = vec![VecDeque::<char>::new(); cols];

    for row in &diagram_lines[1..] {
        for i in 0..cols {
            let row = row.chars().collect::<Vec<_>>();
            let c = row[i * 4 + 1];
            if c != ' ' {
                rows[i].push_back(c);
            }
        }
    }

    // process moves, creating temporary queue
    for line in moves.split("\n") {
        let iter = line.split(" ").collect::<Vec<&str>>();
        let args: Vec<usize> = match iter[..] {
            [_, x, _, y, _, z] => [x, y, z]
                .iter()
                .map(|&c| c.parse().expect(INT_ERR))
                .collect(),
            _ => unreachable!(),
        };

        let (num, src, dest) = match args[..] {
            [num, src, dest] => (num, src, dest),
            _ => unreachable!(),
        };

        // pop from [src] stack
        let mut tmp = VecDeque::<char>::new();
        for _ in 0..num {
            let element = rows[src - 1].pop_back().expect(POP_ERR);
            tmp.push_back(element);
        }

        // perform operations in order specified
        for _ in 0..num {
            let element = match option {
                OperationOrder::FIFO => tmp.pop_front(),
                OperationOrder::FILO => tmp.pop_back(),
            }
            .expect(POP_ERR);
            rows[dest - 1].push_back(element);
        }
    }

    print!("{}", option);
    for row in rows {
        print!("{}", row.back().expect(POP_ERR));
    }
    println!();
}

fn main() {
    solve(OperationOrder::FIFO);
    solve(OperationOrder::FILO);
}
