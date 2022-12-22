use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, Ordering, PartialEq, PartialOrd};

#[derive(Copy, Clone, Debug, Default, PartialEq)] 
struct Resource {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

#[derive(Copy, Clone, Debug, Default)]
struct Blueprint {
    idx: i64,
    ore_costs: Resource,
    clay_costs: Resource,
    obsidian_costs: Resource,
    geode_costs: Resource,
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other { return Some(Ordering::Less); }
        else if self == other { return Some(Ordering::Equal); }
        else if self > other { return Some(Ordering::Greater); }
        None
    }

    fn lt(&self, other: &Self) -> bool {
        self.ore < other.ore && self.clay < other.clay && self.obsidian < other.obsidian && self.geode < other.obsidian
    }

    fn le(&self, other: &Self) -> bool {
        self.ore <= other.ore && self.clay <= other.clay && self.obsidian <= other.obsidian && self.geode <= other.obsidian
    }

    fn gt(&self, other: &Self) -> bool {
        self.ore > other.ore && self.clay > other.clay && self.obsidian > other.obsidian && self.geode > other.obsidian
    }

    fn ge(&self, other: &Self) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian && self.geode >= other.obsidian
    }
}

impl Blueprint {
    fn new(s: &str) -> Blueprint {
        lazy_static! {
            static ref RE : Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs? (\d+) ores?. Each clay robot costs? (\d+) ores?. Each obsidian robot costs? (\d+) ores? and (\d+) clays?. Each geode robot costs (\d+) ores? and (\d+) obsidians?.").unwrap();
        }
        let caps = RE.captures(s).unwrap();

        let idx = caps.get(1).unwrap().as_str().parse().unwrap();
        let ore_ore = 0;
        let clay_ore = 0;
        let obsidian_ore = 0;
        let obsidian_clay = 0;
        let geode_ore = 0;
        let geode_obsidian = 0;

        Blueprint {
            idx,
            ore_costs: Resource {
                ore: ore_ore,
                ..Default::default()
            },
            clay_costs: Resource {
                ore: clay_ore,
                ..Default::default()
            },
            obsidian_costs: Resource {
                ore: obsidian_ore,
                clay: obsidian_clay,
                ..Default::default()
            },
            geode_costs: Resource {
                ore: geode_ore,
                obsidian: geode_obsidian,
                ..Default::default()
            },
        }
    }
}

fn dfs(
    time: i64,
    robots: &Resource,
    resources: &Resource,
    blueprint: &Blueprint,
    best: i64,
) -> i64 {
    if time <= 0 {
        return 0;
    }

    // Assuming every second afterwards is building geode robots
    let max_geode_possible = resources.geode + robots.geode * time + time * (time - 1) / 2;
    if max_geode_possible <= best {
        return 0;
    }

    // Try each option
    let mut cur_best = best;

    // If we can afford each of the robots, try them
    if resources >= &blueprint.geode_costs {
        cur_best = max(cur_best, dfs(time - 1, robots, resources, blueprint, cur_best));
    }

    0
}

pub fn solve(contents: &str) -> (usize, usize) {
    let blueprints: Vec<Blueprint> = contents.lines().map(|l| Blueprint::new(l)).collect();
    
    // println!("r1 > r2? {}", r1 > r2);

    (0, 0)
}
