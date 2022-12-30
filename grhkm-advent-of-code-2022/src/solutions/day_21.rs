use poly::fraction::Fraction;
use poly::poly::{One, Poly, Solvable, Zero};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Expr {
    Val(Fraction),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if s contains all digits
        if let Ok(val) = s.parse::<i128>() {
            return Ok(Expr::Val(Fraction::new(val, 1)));
        }

        // Check if s is empty
        let s = s.trim();
        if s.is_empty() {
            return Err("".to_string());
        }

        // Try to split into three parts
        match s.split_whitespace().collect::<Vec<&str>>()[..] {
            [s1, "+", s2] => Ok(Expr::Add(s1.to_string(), s2.to_string())),
            [s1, "-", s2] => Ok(Expr::Sub(s1.to_string(), s2.to_string())),
            [s1, "*", s2] => Ok(Expr::Mul(s1.to_string(), s2.to_string())),
            [s1, "/", s2] => Ok(Expr::Div(s1.to_string(), s2.to_string())),
            _ => Err(format!("Failed to parse {s} into Expr")),
        }
    }
}

struct MonkeyTree {
    root: Option<String>,
    var: Option<String>,
    tree: HashMap<String, Expr>,
}

impl MonkeyTree {
    fn new() -> MonkeyTree {
        MonkeyTree {
            root: None,
            var: None,
            tree: HashMap::new(),
        }
    }

    fn insert<T>(&mut self, line: T)
    where
        T: Display + Into<String>,
    {
        let line = line.to_string();
        let (name, expr) = line.split_once(": ").unwrap();
        self.tree
            .entry(name.to_string())
            .or_insert(expr.parse().unwrap());
    }

    fn set_root<T>(&mut self, root: T)
    where
        T: Display + Into<String>,
    {
        let root = root.to_string();
        self.root = Some(root);
    }

    fn set_var<T>(&mut self, var: T)
    where
        T: Display + Into<String>,
    {
        let var = var.to_string();
        self.var = Some(var);
    }

    fn eval_node<T>(&self, node: T) -> Poly<Fraction>
    where
        T: Display + Into<String>,
    {
        let node = node.to_string();
        if let Some(var) = &self.var {
            if &node == var {
                println!("[!] Found variable");
                return Poly::new(&vec![Fraction::zero(), Fraction::one()]);
            }
        }

        let expr = self.tree.get(&node);
        if expr.is_none() {
            panic!("Error: tree does not contain node {node}");
        }

        // If `var` is set, we always subtract and return a root
        if self.var.is_some() && Some(node) == self.root {
            println!("OP: {expr:?}");
            return match expr.unwrap() {
                Expr::Add(s1, s2) => self.eval_node(s1) - self.eval_node(s2),
                Expr::Sub(s1, s2) => self.eval_node(s1) - self.eval_node(s2),
                Expr::Mul(s1, s2) => self.eval_node(s1) - self.eval_node(s2),
                Expr::Div(s1, s2) => self.eval_node(s1) - self.eval_node(s2),
                _ => unreachable!(),
            };
        }

        match expr.unwrap() {
            Expr::Val(val) => Poly::from_const(*val),
            Expr::Add(s1, s2) => self.eval_node(s1) + self.eval_node(s2),
            Expr::Sub(s1, s2) => self.eval_node(s1) - self.eval_node(s2),
            Expr::Mul(s1, s2) => self.eval_node(s1) * self.eval_node(s2),
            Expr::Div(s1, s2) => self.eval_node(s1) / self.eval_node(s2),
        }
    }

    fn eval(&self) -> Poly<Fraction> {
        if self.root.is_none() {
            panic!("Error: calling eval while tree root not set");
        }

        if self.var.is_some() {
            panic!("Error: calling eval while tree var is set");
        }

        self.eval_node(self.root.as_ref().unwrap())
    }

    // Finds value for `var` such that two sides of `root` are equal
    fn solve_match(&self) -> Fraction {
        if self.var.is_none() {
            panic!("Error: calling solve_match while tree var is not set");
        }

        let eq = self.eval_node(self.root.as_ref().unwrap());
        println!("EQ: {eq:?}");
        let sols = eq.roots();

        if sols.len() != 1 {
            panic!("Error: calling solve_match returned {eq}, which has {sols:?} as solutions");
        }

        sols[0]
    }
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut tree = MonkeyTree::new();

    for line in contents.lines() {
        tree.insert(line);
    }

    tree.set_root("root");
    let root_eval = tree.eval();
    let part1 = root_eval.get(0usize);

    if root_eval.deg() != 0 || part1.denom() != 1 {
        panic!("Error: {root_eval} is not an integer");
    }
    let part1 = part1.num() as usize;

    tree.set_var("humn");
    let part2 = tree.solve_match();

    if part2.denom() != 1 {
        panic!("Error: {part2} is not an integer");
    }
    let part2 = part2.num() as usize;

    (part1, part2)
}
