use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Clone)]
enum OperationVal {
    Old,
    Val(BigUint),
}

#[derive(Clone)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<BigUint>,
    operation_val: OperationVal,
    operation_type: OperationType,
    test_div: BigUint,
    test_true: usize,
    test_false: usize,
    inspect_cnt: usize,
}

impl Monkey {
    fn get_item(&mut self) -> Option<BigUint> {
        if self.items.is_empty() {
            return None;
        }
        self.inspect_cnt += 1;
        self.items.pop_front()
    }

    fn operation(&self, val: &BigUint) -> BigUint {
        let operation_val = if let OperationVal::Val(operation_val) = &self.operation_val {
            operation_val
        } else {
            val
        };

        match self.operation_type {
            OperationType::Add => val + operation_val,
            OperationType::Sub => val - operation_val,
            OperationType::Mul => val * operation_val,
            OperationType::Div => val / operation_val,
        }
    }

    fn test(&self, val: &BigUint) -> usize {
        if val % &self.test_div == Zero::zero() {
            self.test_true
        } else {
            self.test_false
        }
    }
}

fn simulate(times: usize, div: &BigUint, monkeys_mod: &BigUint, mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..times {
        for i in 0..monkeys.len() {
            while let Some(worry_level) = monkeys[i].get_item() {
                let worry_level = (monkeys[i].operation(&worry_level) / div) % monkeys_mod;
                let throw_to = monkeys[i].test(&worry_level);
                monkeys[throw_to].items.push_back(worry_level.clone());
            }
        }
    }

    let mut inspect_cnts = monkeys
        .iter()
        .map(|m| m.inspect_cnt)
        .collect::<Vec<usize>>();
    inspect_cnts.sort();
    inspect_cnts.reverse();

    inspect_cnts[0] * inspect_cnts[1]
}

pub fn solve(contents: &str) -> (usize, usize) {
    let re: Regex = Regex::new(r"\d+").unwrap();
    let find_int = |s| re.find(s).unwrap().as_str().parse::<usize>().unwrap();
    let mut monkeys_mod: BigUint = 1.to_biguint().unwrap();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_str in contents.split("\n\n") {
        let (items_str, operation_str, test_div, test_true, test_false) = if let [_, items_str, operation_str, test_div_str, test_true_str, test_false_str] =
            monkey_str.split('\n').into_iter().collect::<Vec<&str>>()[..]
        {
            let test_div = find_int(test_div_str);
            let test_true = find_int(test_true_str);
            let test_false = find_int(test_false_str);
            (items_str, operation_str, test_div, test_true, test_false)
        } else {
            panic!();
        };

        // parse comma-separated items
        let mut items = VecDeque::<BigUint>::new();
        for item in items_str.split_once(": ").unwrap().1.split(", ") {
            items.push_back(item.parse().unwrap());
        }

        // parse different types of arithmetic operations
        let operation_tokens = operation_str
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<Vec<&str>>();
        let operation_val = if let Ok(val) = operation_tokens[4].parse::<BigUint>() {
            OperationVal::Val(val)
        } else {
            OperationVal::Old
        };
        let operation_type = match operation_tokens[3] {
            "+" => OperationType::Add,
            "-" => OperationType::Sub,
            "*" => OperationType::Mul,
            "/" => OperationType::Div,
            _ => unreachable!(),
        };

        let test_div = test_div.to_biguint().unwrap();
        monkeys_mod *= &test_div;

        monkeys.push(Monkey {
            items,
            operation_val,
            operation_type,
            test_div,
            test_true,
            test_false,
            inspect_cnt: 0,
        });
    }

    let const_3 = 3.to_biguint().unwrap();
    let const_1 = 1.to_biguint().unwrap();

    let part1 = simulate(20, &const_3, &monkeys_mod, monkeys.clone());
    let part2 = simulate(10000, &const_1, &monkeys_mod, monkeys.clone());

    (part1, part2)
}
