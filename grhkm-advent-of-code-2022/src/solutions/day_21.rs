use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use poly::poly::Poly;

#[derive(Clone, Debug)]
enum Expr {
    Val(i128),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Poly(Poly<i128>),
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if s contains all digits
        if let Ok(val) = s.parse::<i128>() {
            return Ok(Expr::Val(val));
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
    root: String,
    tree: HashMap<String, Expr>,
}

impl MonkeyTree {
    fn new() -> MonkeyTree {
        MonkeyTree {
            root: "".to_string(),
            tree: HashMap::new(),
        }
    }

    // pppw: cczh / lfqf
    fn insert<T>(&mut self, line: T) where T: Display + Into<String> {
        let line = line.to_string();
        let (name, expr) = line.split_once(": ").unwrap();
        self.tree.entry(name.to_string()).or_insert(expr.parse().unwrap());
    }

    fn set_root<T>(&mut self, root: T) where T: Display + Into<String> {
        let root = root.to_string();
        self.root = root;
    }

    fn eval_node<T>(&self, node: T) -> i128 where T: Display + Into<String> {
        let node = node.to_string();
        
        let expr = self.tree.get(&node);
        if expr.is_none() {
            panic!("Error: tree does not contain node {node}");
        }

        match expr.unwrap() {
            Expr::Val(val) => *val,
            Expr::Add(s1, s2) => self.eval_node(s1) + self.eval_node(s2),
            Expr::Sub(s1, s2) => self.eval_node(s1) - self.eval_node(s2),
            Expr::Mul(s1, s2) => self.eval_node(s1) * self.eval_node(s2),
            Expr::Div(s1, s2) => self.eval_node(s1) / self.eval_node(s2),
            Expr::Poly(_poly) => todo!()
        }
    }

    fn eval(&self) -> i128 {
        self.eval_node(&self.root)
    }
}

pub fn solve(contents: &str) -> (usize, usize) {
    let mut tree = MonkeyTree::new();

    for line in contents.lines() {
        tree.insert(line);
    }

    tree.set_root("root");
    let part1 = tree.eval() as usize;
    
    (part1, 0)
}
