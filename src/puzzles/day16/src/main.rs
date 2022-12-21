mod parser;
mod shortest_paths;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use shortest_paths::ShortestPaths;

use crate::{parser::parse_valve, shortest_paths::floyd_warshall_shortest_paths};

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

#[derive(Debug)]
enum Thread {
    Start,
    Extension {
        opened_valve: &'static str,
        minute_opened: u32,
        prev: Rc<Thread>,
    },
}

impl<'a> Thread {
    fn minute_opened(&self) -> u32 {
        match self {
            Self::Start => 0,
            Self::Extension { minute_opened, .. } => *minute_opened,
        }
    }

    fn valve_is_open(&self, valve: &'static str) -> bool {
        match self {
            Self::Start => false,
            Self::Extension {
                opened_valve, prev, ..
            } => *opened_valve == valve || prev.valve_is_open(valve),
        }
    }

    fn score(&self, valve_lookup: &ValveLookup, total_runtime: u32) -> u32 {
        match self {
            Self::Start => 0,
            Self::Extension {
                opened_valve,
                minute_opened,
                prev,
            } => {
                let valve = valve_lookup.get(opened_valve).unwrap();
                valve.flow_rate * (total_runtime - minute_opened)
                    + prev.score(valve_lookup, total_runtime)
            }
        }
    }

    fn backtracking_search<F: FnMut(&Rc<Thread>) -> bool>(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
        is_potential_solution: &mut F,
    ) {
        if is_potential_solution(self) {
            for extension in self.extensions(shortest_paths, total_runtime) {
                extension.backtracking_search(shortest_paths, total_runtime, is_potential_solution);
            }
        }
    }

    fn reachable_closed_valves(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
    ) -> HashMap<&'static str, u32> {
        let current_valve = match self.borrow() {
            Thread::Start => "AA",
            Thread::Extension { opened_valve, .. } => opened_valve,
        };
        shortest_paths
            .all_shortest_paths_from(current_valve)
            .into_iter()
            .flatten()
            .filter_map(move |(target, path_length)| {
                let minute_opened = self.minute_opened() + path_length + 1;
                let is_valid = minute_opened < total_runtime && !self.valve_is_open(target);
                is_valid.then_some((*target, minute_opened))
            })
            .collect()
    }

    fn extensions(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
    ) -> Vec<Rc<Thread>> {
        self.reachable_closed_valves(shortest_paths, total_runtime)
            .into_iter()
            .map(|(target, minute_opened)| {
                Rc::new(Thread::Extension {
                    minute_opened,
                    opened_valve: target,
                    prev: self.clone(),
                })
            })
            .collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let valve_lookup: ValveLookup = input.lines().map(parse_valve).collect();
    let shortest_paths =
        floyd_warshall_shortest_paths(&valve_lookup).filter_out_faulty_valves(&valve_lookup);

    let start = Rc::new(Thread::Start);

    let mut part_1_answer = 0;
    let total_runtime = 30;
    start.backtracking_search(&shortest_paths, total_runtime, &mut |thread| {
        let score = thread.score(&valve_lookup, total_runtime);
        part_1_answer = u32::max(part_1_answer, score);
        true
    });
    println!("part 1: {part_1_answer}");
}
