mod parser;
mod shortest_paths;
mod valve_path;
mod valve_thread;
use std::collections::{BinaryHeap, HashMap, HashSet};

use shortest_paths::ShortestPaths;

use crate::{
    parser::parse_valve, shortest_paths::floyd_warshall_shortest_paths, valve_path::ValvePath,
};

pub type ValveLookup = HashMap<&'static str, Valve>;

#[derive(Debug)]
pub struct Valve {
    name: &'static str,
    flow_rate: u32,
    neighbours: HashSet<&'static str>,
}

impl Valve {
    fn new(
        name: &'static str,
        flow_rate: &str,
        neighbours: impl Iterator<Item = &'static str>,
    ) -> Self {
        Self {
            name,
            flow_rate: flow_rate.parse().unwrap(),
            neighbours: neighbours.collect(),
        }
    }
}

struct PathCollection {
    candidate_paths: BinaryHeap<ValvePath>,
    best_path: Option<ValvePath>,
}

impl PathCollection {
    fn new(
        start_valve: &Valve,
        valve_lookup: &ValveLookup,
        shortest_paths: &ShortestPaths,
        total_minutes: u32,
        thread_count: usize,
    ) -> Self {
        let path = ValvePath::new(
            start_valve,
            shortest_paths,
            valve_lookup,
            thread_count,
            total_minutes,
        );
        let mut candidate_paths = BinaryHeap::new();
        candidate_paths.push(path);
        Self {
            candidate_paths,
            best_path: None,
        }
    }

    fn best_score(&self) -> u32 {
        self.best_path
            .as_ref()
            .map(|best_path| best_path.score)
            .unwrap_or(0)
    }

    fn extend_candidate_paths(
        &mut self,
        shortest_paths: &ShortestPaths,
        valve_lookup: &ValveLookup,
        total_minutes: u32,
    ) {
        let mut prev_candidate_paths = std::mem::take(&mut self.candidate_paths);
        while let Some(old_candidate_path) = prev_candidate_paths.pop() {
            if old_candidate_path.score_upper_bound > self.best_score() {
                if old_candidate_path.done {
                    self.candidate_paths.push(old_candidate_path);
                } else {
                    let mut extended_paths = old_candidate_path.all_possible_extensions(
                        total_minutes,
                        valve_lookup,
                        shortest_paths,
                    );

                    while let Some(extended_path) = extended_paths.pop() {
                        if extended_path.score_upper_bound > self.best_score() {
                            let extended_path_score = extended_path.score;
                            if extended_path_score > self.best_score() {
                                self.best_path = Some(extended_path.clone());
                            }
                            self.candidate_paths.push(extended_path);
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

fn part_one(valve_lookup: &ValveLookup, shortest_paths: &ShortestPaths) -> u32 {
    let start_valve = valve_lookup.get("AA").unwrap();

    let runtime = 30;
    let mut paths = PathCollection::new(start_valve, valve_lookup, shortest_paths, runtime, 1);

    while paths.candidate_paths.iter().any(|path| !path.done) {
        println!("extending...path count: {}", paths.candidate_paths.len());
        paths.extend_candidate_paths(shortest_paths, valve_lookup, runtime);
    }

    paths.best_score()
}

// fn part_two(valve_lookup: &ValveLookup, shortest_paths: &ShortestPaths) -> u32 {
//     let start_valve = valve_lookup.get("AA").unwrap();

//     let runtime = 26;
//     let mut paths = PathCollection::new(start_valve, valve_lookup, shortest_paths, runtime, 2);

//     for minute in 1..=runtime {
//         println!(
//             "minute: {minute}, path count: {}",
//             paths.candidate_paths.len()
//         );
//         paths.extend_candidate_paths(shortest_paths, valve_lookup, runtime);
//     }

//     paths.best_score()
// }

fn main() {
    let input = include_str!("../input.txt");
    let valve_lookup: ValveLookup = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let part_1_answer = part_one(&valve_lookup, &shortest_paths);
    println!("part 1: {part_1_answer}");

    // let part_2_answer = part_two(&valve_lookup, &shortest_paths);
    // println!("part 2: {part_2_answer}");
}
