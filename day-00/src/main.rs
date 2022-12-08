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

    fn last_child(&self) -> &TreeNode<T> {
        if self.children.is_empty() {
            panic!("{}", TREE_ERR);
        }
        &self.children[self.children.len() - 1]
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

    fn find_child_default(&mut self, name: impl ToString + Copy) -> &TreeNode<T> {
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
        res.push(f(self));
        for child in &self.children {
            // workaround for infinite recursion
            let f = &f as &dyn Fn(&TreeNode<T>) -> T2;
            res.extend(child.dfs(f));
        }
        res
    }
}

fn main() {
    let mut root = TreeNode::new("root", Some(1000));
    root.insert("child", Some(100));

    let child = root.find_child("child").unwrap();
    child.insert("subchild1", Some(1));
    child.insert("subchild2", Some(2));
    child.insert("subchild3", Some(3));

    let subchild = child.find_child("subchild2").unwrap();
    subchild.insert("subsubchild!", Some(-5));

    println!("{:?}", root);

    let sums = root.dfs(|v| v.sum());
    for v in sums {
        println!("{:?}", v);
    }
}
