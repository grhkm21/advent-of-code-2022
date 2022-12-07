use std::cell::RefCell;
use std::default::Default;
use std::fmt::Debug;
use std::ops::Add;
use std::rc::Rc;

const PARSE_ERR: &str = "err: can't parse int";
const SPLIT_ERR: &str = "err: can't split string";

struct Solver<'a> {
    content: &'a str,
}

trait Solvable {
    fn solve(self) -> (usize, usize);
}

#[derive(Debug)]
struct TreeNode<T: Add<Output = T> + Default + Copy + Debug> {
    val: Option<T>,
    name: String,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Add<Output = T> + Default + Copy + Debug> TreeNode<T> {
    fn new(s: String) -> TreeNode<T> {
        TreeNode {
            val: None,
            name: s,
            children: Vec::new(),
        }
    }

    fn val(&mut self, val: T) {
        self.val = Some(val);
    }

    fn push(&mut self, node: TreeNode<T>) {
        self.children.push(Rc::new(RefCell::new(node)));
    }

    fn insert_and_find(&mut self, name: String) -> Rc<RefCell<TreeNode<T>>> {
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

    // I can't get it to return the TreeNode itself
    // Returning the .name is best I can do
    fn dfs<F, T2>(&self, f: F) -> Vec<T2>
    where
        F: Fn(&TreeNode<T>) -> T2,
    {
        let mut res = Vec::new();
        res.push(f(self));
        for child in &self.children {
            let child = child.borrow_mut();
            for val in child.dfs(&f as &dyn Fn(&TreeNode<T>) -> T2) {
                res.push(val);
            }
        }
        res
    }
}

impl<T: Add<Output = T> + Default + Copy + Debug> Default for TreeNode<T> {
    fn default() -> Self {
        Self::new("".to_string())
    }
}

impl<'a> Solvable for Solver<'a> {
    fn solve(self) -> (usize, usize) {
        let root: Rc<RefCell<TreeNode<usize>>> = Default::default();
        let mut cur_len = 0;
        let mut cur_path = Vec::new();
        cur_path.push(Rc::clone(&root));

        for cmds in self.content.split("$ ") {
            let cur = &cur_path[cur_len];
            let lines: Vec<&str> = cmds.trim().lines().collect();
            if lines.is_empty() {
                continue;
            }

            let cmd: Vec<&str> = lines[0].split(" ").collect();
            if cmd[0] == "ls" {
                for &line in &lines[1..] {
                    let (prefix, dir) = line.split_once(" ").expect(SPLIT_ERR);
                    let dir = dir.to_string();
                    if prefix == "dir" {
                        cur.borrow_mut().insert_and_find(dir);
                    } else {
                        let size: usize = prefix.parse().expect(PARSE_ERR);
                        let child = cur.borrow_mut().insert_and_find(dir);
                        child.borrow_mut().val(size);
                    }
                }
            } else {
                match cmd[..] {
                    ["cd", ".."] => {
                        cur_len -= 1;
                        cur_path.pop();
                    }
                    ["cd", dir] => {
                        let child = cur.borrow_mut().insert_and_find(dir.to_string());
                        cur_len += 1;
                        cur_path.push(child);
                    }
                    _ => unreachable!(),
                }
            }
        }

        let func = |node: &TreeNode<usize>| {
            (
                if let Some(_) = node.val { true } else { false },
                node.sum(),
            )
        };
        let root = root.borrow_mut();
        let vals = root.dfs(func);

        // part 1
        let mut part1 = 0;
        for (is_file, sum) in vals.clone() {
            // println!(
            //     "[{}] name = {name:?}, sum = {sum:?}",
            //     if is_file { "-" } else { "+" }
            // );
            if is_file {
                continue;
            }
            if sum <= 100000 {
                part1 += sum;
            }
        }

        // part 2
        let freeup = root.sum() - 40000000;
        let mut part2 = usize::MAX;
        for (is_file, sum) in vals.clone() {
            if is_file {
                continue;
            }
            if sum >= freeup && sum < part2 {
                part2 = sum;
            }
        }

        (part1, part2)
    }
}

fn main() {
    let solver = Solver {
        content: include_str!("../input"),
    };
    let (part1, part2) = solver.solve();
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}
