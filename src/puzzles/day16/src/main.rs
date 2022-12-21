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

    fn for_each_extension<F: FnMut(Rc<Thread>)>(
        self: Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
        visit: &mut F,
    ) {
        for extension in self.extensions(shortest_paths, total_runtime) {
            extension.for_each_extension(shortest_paths, total_runtime, visit);
        }
        visit(self);
    }

    fn extensions(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
    ) -> Vec<Rc<Thread>> {
        let current_valve = match self.borrow() {
            Thread::Start => "AA",
            Thread::Extension { opened_valve, .. } => opened_valve,
        };
        shortest_paths
            .all_shortest_paths_from(current_valve)
            .unwrap()
            .iter()
            .filter_map(move |(target, path_length)| {
                let minute_opened = self.minute_opened() + path_length + 1;
                let can_open_valve = minute_opened < total_runtime && !self.valve_is_open(target);
                can_open_valve.then_some(Rc::new(Thread::Extension {
                    minute_opened,
                    opened_valve: target,
                    prev: self.clone(),
                }))
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
    start.for_each_extension(&shortest_paths, total_runtime, &mut |thread| {
        part_1_answer = u32::max(part_1_answer, thread.score(&valve_lookup, total_runtime));
    });
    println!("part 1: {part_1_answer}");
}
