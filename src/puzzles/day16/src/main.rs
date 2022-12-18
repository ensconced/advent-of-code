mod parser;
mod shortest_paths;
mod single_path;
mod valve_thread;
use std::collections::{BinaryHeap, HashMap, HashSet};

use shortest_paths::ShortestPaths;
use utils::read_input;

use crate::{
    parser::parse_valve, shortest_paths::floyd_warshall_shortest_paths, single_path::ValvePath,
};

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    neighbours: HashSet<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(name: &'a str, flow_rate: &str, neighbours: impl Iterator<Item = &'a str>) -> Self {
        Self {
            name,
            flow_rate: flow_rate.parse().unwrap(),
            neighbours: neighbours.collect(),
        }
    }
}

struct PathCollection<'a> {
    paths: BinaryHeap<ValvePath<'a>>,
    best_score: u32,
}

impl<'a> PathCollection<'a> {
    fn new(
        start_valve: &'a Valve,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
    ) -> Self {
        let path = ValvePath::new(start_valve, shortest_paths, valve_lookup);
        let mut paths = BinaryHeap::new();
        paths.push(path);
        Self {
            paths,
            best_score: 0,
        }
    }

    fn extend(
        &mut self,
        shortest_paths: &ShortestPaths,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        minute: u32,
    ) {
        let mut old_paths = std::mem::take(&mut self.paths);

        while let Some(old_path) = old_paths.pop() {
            if old_path.score_upper_bound > self.best_score {
                if old_path.done {
                    self.paths.push(old_path);
                } else {
                    let mut extended_paths =
                        old_path.all_possible_extensions(minute, valve_lookup, shortest_paths);

                    while let Some(extended_path) = extended_paths.pop() {
                        if extended_path.score_upper_bound > self.best_score {
                            let extended_path_score = extended_path.score;
                            if extended_path_score > self.best_score {
                                self.best_score = extended_path_score;
                            }
                            self.paths.push(extended_path);
                        } else {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }
    }
}

pub const MINUTES: u32 = 30;

fn part_one(valve_lookup: &HashMap<&str, Valve>, shortest_paths: &ShortestPaths) -> u32 {
    let start_valve = valve_lookup.get("AA").unwrap();

    let mut paths = PathCollection::new(start_valve, valve_lookup, shortest_paths);

    for minute in 1..=MINUTES {
        println!("minute: {minute}, path count: {}", paths.paths.len());
        paths.extend(shortest_paths, valve_lookup, minute);
    }

    paths.best_score
}

fn main() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let part_1_answer = part_one(&valve_lookup, &shortest_paths);
    println!("part 1: {part_1_answer}");
}
