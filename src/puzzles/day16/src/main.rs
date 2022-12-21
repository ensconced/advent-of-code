mod parser;
mod shortest_paths;
mod thread;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{parser::parse_valve, shortest_paths::floyd_warshall_shortest_paths, thread::Thread};

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

fn main() {
    let input = include_str!("../input.txt");
    let valve_lookup: ValveLookup = input.lines().map(parse_valve).collect();
    let shortest_paths =
        floyd_warshall_shortest_paths(&valve_lookup).filter_out_faulty_valves(&valve_lookup);

    let start = Rc::new(Thread::Start);

    let mut part_1_answer = 0;
    let total_runtime = 30;
    start.pruning_search(&shortest_paths, total_runtime, &mut |thread| {
        let score = thread.score(&valve_lookup, total_runtime);
        part_1_answer = u32::max(part_1_answer, score);
        thread.score_upper_bound(&shortest_paths, total_runtime, &valve_lookup) > part_1_answer
    });
    println!("part 1: {part_1_answer}");
}
