use std::default::Default;
use std::fmt::Debug;
use std::ops::Add;

const TREE_ERR: &str = "err: tree children is empty";
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

#[derive(Clone, Debug)]
struct TreeNode<T: NodeValTrait<T>> {
    name: String,
    val: Option<T>,
    children: Vec<TreeNode<T>>,
}

impl<T: NodeValTrait<T>> TreeNode<T> {
    fn new(name: impl ToString, val: Option<T>) -> TreeNode<T> {
        TreeNode {
            val: val,
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    fn set_val(&mut self, val: T) {
        self.val = Some(val);
    }

    fn push(&mut self, node: TreeNode<T>) {
        self.children.push(node);
    }

    fn insert(&mut self, name: impl ToString, val: Option<T>) {
        let name = name.to_string();
        for child in &self.children {
            if child.name == name {
                return;
            }
        }

        let child = TreeNode::new(name, val);
        self.push(child);
    }

    fn last_child(&mut self) -> &mut TreeNode<T> {
        if self.children.is_empty() {
            panic!("{}", TREE_ERR);
        }
        let len = self.children.len();
        &mut self.children[len - 1]
    }

    fn find_child(&mut self, name: impl ToString) -> Option<&mut TreeNode<T>> {
        let name = name.to_string();
        for child in &mut self.children {
            if child.name == name {
                return Some(child);
            }
        }
        None
    }

    fn find_child_default(&mut self, name: impl ToString + Copy) -> &mut TreeNode<T> {
        if self.find_child(name).is_some() {
            self.find_child(name).unwrap()
        } else {
            self.insert(name, None);
            self.last_child()
        }
    }

    fn sum(&self) -> T {
        let mut res: T = Default::default();
        if let Some(val) = &self.val {
            res = res + *val;
        }
        for child in &self.children {
            res = res + child.sum();
        }
        res
    }

    fn dfs<F, T2>(&self, f: F) -> Vec<T2>
    where
        F: Fn(&TreeNode<T>) -> T2,
    {
        let mut res = Vec::new();
        println!("cur: dfs at {self:?}");
        res.push(f(self));
        for child in &self.children {
            // workaround for infinite recursion
            let f = &f as &dyn Fn(&TreeNode<T>) -> T2;
            res.extend(child.dfs(f));
        }
        res
    }
}

impl Solvable for Solver {
    fn solve(self) -> (usize, usize) {
        let root = TreeNode::new("root", None);
        let mut cur_path = Vec::new();
        cur_path.push(root.clone());

        // each $ represents new command input / output group
        for cmds in self.content.split("$ ") {
            let mut cur = cur_path[cur_path.len() - 1].clone();
            let lines: Vec<&str> = cmds.trim().lines().collect();
            if lines.is_empty() {
                continue;
            }

            // process each group of commands
            let cmd: Vec<&str> = lines[0].split(" ").collect();
            if cmd[0] == "ls" {
                for &line in &lines[1..] {
                    let (prefix, dir) = line.split_once(" ").expect(SPLIT_ERR);
                    let child = cur.find_child_default(dir);
                    match prefix {
                        "dir" => {}
                        _ => {
                            let prefix = prefix.parse::<usize>().expect(PARSE_ERR);
                            child.set_val(prefix);
                        }
                    }
                    println!(
                        "child = {:?}, cur = {:?}",
                        child,
                        cur_path[cur_path.len() - 1].clone()
                    );
                }
            } else {
                match cmd[1] {
                    ".." => {
                        cur_path.pop();
                    }
                    dir => {
                        let child = cur.find_child_default(dir).clone();
                        cur_path.push(child);
                    }
                }
            }
        }

        let func = |node: &TreeNode<usize>| (node.val.is_some(), node.sum());
        let binding = root.dfs(func);
        let vals = binding
            .iter()
            .filter(|(is_file, _)| !is_file)
            .map(|(_, sum)| sum);

        // part 1
        let part1 = vals.clone().filter(|&sum| *sum <= 100000).sum();
        // println!("part1: {part1:?}");

        // part 2
        let freeup = root.sum() - 40000000;
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
