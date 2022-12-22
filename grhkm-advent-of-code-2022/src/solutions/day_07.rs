use std::cell::RefCell;
use std::default::Default;
use std::fmt::Debug;
use std::ops::Add;
use typed_arena::Arena;

// Blanket implementation
trait NodeValTrait<T> = Add<Output = T> + Default + Copy + Debug;

#[derive(Debug)]
struct Node<'a, T>
where
    T: NodeValTrait<T>,
{
    val: Option<T>,
    name: &'a str,
    edges: RefCell<Vec<&'a Node<'a, T>>>,
}

impl<'a, T> Node<'a, T>
where
    T: NodeValTrait<T>,
{
    fn new<'b>(val: Option<T>, name: &'b str, arena: &'b Arena<Node<'b, T>>) -> &'b Node<'b, T> {
        arena.alloc(Node {
            val,
            name,
            edges: RefCell::new(Vec::new()),
        })
    }

    fn push(&'a self, node: &'a Node<'a, T>) {
        (*self.edges.borrow_mut()).push(node);
    }

    fn find_name<'b>(&'a self, name: &'b str) -> Option<&'a Node<'a, T>> {
        (*self.edges.borrow())
            .iter()
            .find(|&node| node.name == name)
            .copied()
    }

    fn sum(&self) -> T {
        let mut res = Default::default();
        if let Some(val) = self.val {
            res = res + val;
        }
        for node in &(*self.edges.borrow()) {
            res = res + node.sum();
        }
        res
    }

    fn traverse<F, F2>(&self, f: &F) -> Vec<F2>
    where
        F: Fn(&Self) -> F2,
    {
        let mut res = Vec::new();
        res.push(f(self));
        for n in &(*self.edges.borrow()) {
            res.extend(n.traverse(f));
        }
        res
    }
}

fn init<'a>(arena: &'a Arena<Node<'a, usize>>, contents: &'a str) -> &'a Node<'a, usize> {
    let cur_path: &mut Vec<&'a Node<'a, usize>> = &mut Vec::new();
    cur_path.clear();
    let root = Node::new(None, "/", arena);
    cur_path.push(root);

    for cmd_group in contents.split("$ ") {
        let cmd_group: Vec<&str> = cmd_group.trim().lines().collect();
        if cmd_group.is_empty() {
            continue;
        }

        let cmd = cmd_group[0]
            .split_once(' ')
            .unwrap_or_else(|| (cmd_group[0], ""));
        match cmd {
            ("ls", "") => {
                for output in &cmd_group[1..] {
                    let (data, dir) = output.split_once(' ').unwrap();

                    // assume node is new
                    let cur = cur_path[cur_path.len() - 1];

                    let val = match data {
                        "dir" => None,
                        data => Some(data.parse().unwrap()),
                    };

                    match cur.find_name(dir) {
                        Some(_) => {
                            todo!()
                        }
                        None => {
                            let node = Node::new(val, dir, arena);
                            cur.push(node);
                        }
                    }
                }
            }
            ("cd", "..") => {
                cur_path.pop();
            }
            ("cd", "/") => {
                cur_path.clear();
                cur_path.push(root);
            }
            ("cd", dir) => {
                // assume node is new
                let cur = cur_path[cur_path.len() - 1];
                let node = match cur.find_name(dir) {
                    None => {
                        let node = Node::new(None, dir, arena);
                        cur.push(node);
                        node
                    }
                    Some(node) => node,
                };
                cur_path.push(node);
            }
            _ => unreachable!(),
        }
    }
    root
}

pub fn solve(contents: &str) -> (usize, usize) {
    let arena = Arena::new();

    // construct graph
    let g = init(&arena, contents);
    let res = g
        .traverse(&|v| {
            // println!("{:?} -> {:?}", v, v.sum());
            (v.val.is_none(), v.sum())
        })
        .iter()
        .filter(|(is_dir, _)| *is_dir)
        .map(|(_, sum)| *sum)
        .collect::<Vec<_>>();

    let sum: usize = g.sum();
    let part1: usize = res.iter().filter(|&&s| s <= 100000).sum();
    let part2: usize = *res
        .iter()
        .filter(|&&s| s >= sum - 40000000)
        .min()
        .expect("err: no files can be freed");

    (part1, part2)
}
