mod parser;
mod shortest_paths;
mod valve_path;
mod valve_thread;
use std::collections::{HashMap, HashSet};

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
enum Thread<'a> {
    Start,
    Extension {
        opened_valve: &'static str,
        minute_opened: u32,
        prev: &'a Thread<'a>,
    },
}

impl<'a> Thread<'a> {
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

    fn score(&self, valve_lookup: &ValveLookup) -> u32 {
        match self {
            Self::Start => 0,
            Self::Extension {
                opened_valve,
                minute_opened,
                prev,
            } => {
                let valve = valve_lookup.get(opened_valve).unwrap();
                valve.flow_rate * (30 - minute_opened) + prev.score(valve_lookup)
            }
        }
    }

    fn visit_recursively<F: Fn(&Thread)>(&self, shortest_paths: &'a ShortestPaths, visit: &F) {
        visit(self);
        // TODO - add bounds check here?
        for extension in self.extensions(shortest_paths) {
            extension.visit_recursively(shortest_paths, visit);
        }
    }

    fn extensions(&'a self, shortest_paths: &'a ShortestPaths) -> impl Iterator<Item = Thread<'a>> {
        let current_valve = match self {
            Self::Start => "AA",
            Self::Extension { opened_valve, .. } => opened_valve,
        };
        shortest_paths
            .all_shortest_paths_from(current_valve)
            .unwrap()
            .iter()
            .filter_map(|(target, path_length)| {
                let minute_opened = self.minute_opened() + path_length + 1;
                let can_open_valve = minute_opened < 30 && !self.valve_is_open(target);
                can_open_valve.then_some(Thread::Extension {
                    minute_opened,
                    opened_valve: target,
                    prev: self,
                })
            })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let valve_lookup: ValveLookup = input.lines().map(parse_valve).collect();
    let shortest_paths =
        floyd_warshall_shortest_paths(&valve_lookup).filter_out_faulty_valves(&valve_lookup);

    let start = Thread::Start;
    start.visit_recursively(&shortest_paths, &|asdf| {
        println!("{}", asdf.score(&valve_lookup));
    });
}
