use std::cell::RefCell;
use std::default::Default;
use std::fmt::Debug;
use std::ops::Add;
use std::rc::Rc;

const ITER_ERR: &str = "err: iterator is empty";
const PARSE_ERR: &str = "err: can't parse int";
const SPLIT_ERR: &str = "err: can't split string";

struct Solver {
    content: String,
}

trait Solvable {
    fn solve(self) -> (usize, usize);
}

// Blanket implementation
trait NodeValTrait<T>: Add<Output = T> + Default + Copy + Debug {}

impl<T> NodeValTrait<T> for T where T: Add<Output = T> + Default + Copy + Debug {}

#[derive(Debug)]
struct TreeNode<T: NodeValTrait<T>> {
    val: Option<T>,
    name: String,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: NodeValTrait<T>> TreeNode<T> {
    fn new(s: impl ToString) -> TreeNode<T> {
        TreeNode {
            val: None,
            name: s.to_string(),
            children: Vec::new(),
        }
    }

    fn val(&mut self, val: T) {
        self.val = Some(val);
    }

    fn push(&mut self, node: TreeNode<T>) {
        self.children.push(Rc::new(RefCell::new(node)));
    }

    fn insert_and_find(&mut self, name: impl ToString) -> Rc<RefCell<TreeNode<T>>> {
        let name = name.to_string();
        for child in &self.children {
            if child.borrow().name == name {
                return Rc::clone(child);
            }
        }
        let child = TreeNode::new(name);
        self.push(child);
        Rc::clone(&self.children[&self.children.len() - 1])
    }

    fn sum(&self) -> T {
        let mut res: T = Default::default();
        if let Some(val) = &self.val {
            res = res + *val;
        }
        for child in &self.children {
            res = res + child.borrow().sum();
        }
        res
    }

    fn dfs<F, T2>(&self, f: F) -> Vec<T2>
    where
        F: Fn(&TreeNode<T>) -> T2,
    {
        let mut res = Vec::new();
        res.push(f(self));
        for child in &self.children {
            // workaround for infinite recursion
            let f = &f as &dyn Fn(&TreeNode<T>) -> T2;
            res.extend(child.borrow_mut().dfs(f));
        }
        res
    }
}

impl Solvable for Solver {
    fn solve(self) -> (usize, usize) {
        let root: Rc<RefCell<TreeNode<usize>>> = Rc::new(RefCell::new(TreeNode::new("")));
        let mut cur_path = Vec::new();
        cur_path.push(Rc::clone(&root));

        // each $ represents new command input / output group
        for cmds in self.content.split("$ ") {
            let cur = &cur_path[cur_path.len() - 1];
            let lines: Vec<&str> = cmds.trim().lines().collect();
            if lines.is_empty() {
                continue;
            }

            // process each group of commands
            let cmd: Vec<&str> = lines[0].split(" ").collect();
            if cmd[0] == "ls" {
                for &line in &lines[1..] {
                    let (prefix, dir) = line.split_once(" ").expect(SPLIT_ERR);
                    let child = cur.borrow_mut().insert_and_find(dir);
                    match prefix {
                        "dir" => {
                            cur.borrow_mut().insert_and_find(dir);
                        }
                        _ => {
                            let prefix = prefix.parse().expect(PARSE_ERR);
                            child.borrow_mut().val(prefix);
                        }
                    }
                }
            } else {
                match cmd[1] {
                    ".." => {
                        cur_path.pop();
                    }
                    dir => {
                        let child = cur.borrow_mut().insert_and_find(dir);
                        cur_path.push(child);
                    }
                }
            }
        }

        let func = |node: &TreeNode<usize>| (node.val.is_some(), node.sum());
        let binding = root.borrow_mut().dfs(func);
        let vals = binding
            .iter()
            .filter(|(is_file, _)| !is_file)
            .map(|(_, sum)| sum);

        // part 1
        let part1 = vals.clone().filter(|&sum| *sum <= 100000).sum();

        // part 2
        let freeup = root.borrow_mut().sum() - 40000000;
        let part2 = vals
            .clone()
            .filter(|&sum| *sum >= freeup)
            .min()
            .expect(ITER_ERR);

        (part1, *part2)
    }
}

fn main() {
    let solver = Solver {
        content: include_str!("../input").to_string(),
    };
    let (part1, part2) = solver.solve();
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}
