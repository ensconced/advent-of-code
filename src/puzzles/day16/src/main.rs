mod parser;
mod shortest_paths;
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

// struct AllExtensions<'a> {
//     stack: Vec<Thread<'a>>,
//     shortest_paths: &'a ShortestPaths,
//     total_runtime: u32,
// }

// impl<'a> AllExtensions<'a> {
//     fn new(shortest_paths: &'a ShortestPaths, total_runtime: u32) -> Self {
//         let stack = vec![Thread::Start];
//         let layer =

//         let last_thread = &stack[stack.len() - 1];
//         let extensions = last_thread.extensions(shortest_paths, total_runtime);

//         Self {
//             stack,
//             shortest_paths,
//             total_runtime,
//         }
//     }
// }

// impl<'a> Iterator for Thread<'a> {
//     type Item = Thread<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(thread) = self.stack.pop() {
//             for extended_thread in thread.extensions(self.shortest_paths, self.total_runtime) {
//                 self.stack.push(extended_thread);
//             }
//             return Some(thread);
//         }
//         None
//     }
// }

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

    fn for_each_extension<F: FnMut(&Thread)>(
        &self,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
        visit: &mut F,
    ) {
        for extension in self.extensions(shortest_paths, total_runtime) {
            extension.for_each_extension(shortest_paths, total_runtime, visit);
        }
        visit(self);
    }

    fn extensions(&'a self, shortest_paths: &'a ShortestPaths, total_runtime: u32) -> Vec<Thread> {
        let current_valve = match self {
            Self::Start => "AA",
            Self::Extension { opened_valve, .. } => opened_valve,
        };
        shortest_paths
            .all_shortest_paths_from(current_valve)
            .unwrap()
            .iter()
            .filter_map(move |(target, path_length)| {
                let minute_opened = self.minute_opened() + path_length + 1;
                let can_open_valve = minute_opened < total_runtime && !self.valve_is_open(target);
                can_open_valve.then_some(Thread::Extension {
                    minute_opened,
                    opened_valve: target,
                    prev: self,
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

    let start = Thread::Start;

    let mut part_1_answer = 0;
    let total_runtime = 30;
    start.for_each_extension(&shortest_paths, total_runtime, &mut |thread| {
        part_1_answer = u32::max(part_1_answer, thread.score(&valve_lookup, total_runtime));
    });
    println!("part 1: {part_1_answer}");
}
