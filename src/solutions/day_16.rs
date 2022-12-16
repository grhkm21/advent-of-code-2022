#![allow(clippy::needless_range_loop)]

use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;

struct Data {
    name: String,
    val: usize,
    valves: Vec<String>,
}

lazy_static! {
    static ref PARSE: Regex =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
}

impl Data {
    fn new(s: &str) -> Data {
        let caps = PARSE.captures(s).unwrap();
        let name = caps.get(1).unwrap();
        let val = caps.get(2).unwrap();
        let valves = caps.get(3).unwrap();
        Data {
            name: name.as_str().to_string(),
            val: val.as_str().parse::<usize>().unwrap(),
            valves: valves.as_str().split(", ").map(|s| s.to_string()).collect(),
        }
    }
}

fn calc_dist(graph: &Vec<Vec<usize>>, dist: &mut Vec<Vec<usize>>) {
    let n = graph.len();
    *dist = vec![vec![1000000; n]; n];
    for u in 0..n {
        for &v in &graph[u] {
            dist[u][v] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
}

fn dfs(
    cur: usize,
    rem: usize,
    t: usize,
    part2: bool,
    dist: &Vec<Vec<usize>>,
    vals: &Vec<usize>,
    positive: &Vec<usize>,
) -> usize {
    let mut res = 0;
    if part2 {
        res = dfs(0, rem, 26, false, dist, vals, positive);
    }
    for (i, &dt) in positive.iter().enumerate() {
        if (rem & (1 << i)) == 0 && dist[cur][dt] < t {
            let new_t = t - dist[cur][dt] - 1;
            let new_rem = rem | (1 << i);
            let mut new_res = vals[dt] * new_t;
            new_res += dfs(dt, new_rem, new_t, part2, dist, vals, positive);
            res = max(res, new_res);
        }
    }
    res
}

pub fn solve(contents: &str) -> (usize, usize) {
    let datas = contents.lines().map(Data::new);

    // map strings to indices
    let name_mp = RefCell::new(HashMap::<String, usize>::new());
    let mut positive = Vec::new();
    let mut graph = Vec::new();
    let mut vals = Vec::new();

    let name_to_idx = |s: String| *name_mp.borrow().get(&s).unwrap();

    for (i, data) in datas.clone().enumerate() {
        name_mp.borrow_mut().insert(data.name, i);
    }

    for (i, data) in datas.enumerate() {
        vals.push(data.val);
        name_mp.borrow_mut().insert(data.name, i);
        if data.val > 0 {
            positive.push(i);
        }

        let mut edges = Vec::new();
        for valve in data.valves {
            edges.push(name_to_idx(valve));
        }
        graph.push(edges);
    }

    let mut dist = Vec::new();
    calc_dist(&graph, &mut dist);

    let n = graph.len();

    let src = name_to_idx("AA".to_string());
    let part1 = dfs(src, 0, 30, false, &dist, &vals, &positive);
    let part2 = dfs(src, 0, 26, true, &dist, &vals, &positive);

    (part1, part2)
}
