mod parser;
mod shortest_paths;
mod thread;
mod utils;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use shortest_paths::ShortestPaths;

use crate::{
    parser::parse_valve,
    shortest_paths::floyd_warshall_shortest_paths,
    thread::{all_opened_valves, max_remaining_value, pruning_search, Thread},
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

fn earliest_times_to_open_reachable_valves(
    thread_set: &[Rc<Thread>],
    shortest_paths: &ShortestPaths,
    total_runtime: u32,
) -> HashMap<&'static str, u32> {
    let already_opened_valves = all_opened_valves(thread_set);

    thread_set
        .iter()
        .map(|thread| {
            thread.earliest_times_to_open_reachable_closed_valves(
                shortest_paths,
                total_runtime,
                &already_opened_valves,
            )
        })
        .reduce(|acc, thread_reachable_closed_valves| {
            acc.keys()
                .chain(thread_reachable_closed_valves.keys())
                .unique()
                .map(|valve_name| {
                    let earliest_valve_could_be_opened = acc
                        .get(valve_name)
                        .into_iter()
                        .chain(thread_reachable_closed_valves.get(valve_name))
                        .min()
                        .cloned()
                        .unwrap();

                    (*valve_name, earliest_valve_could_be_opened)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn main() {
    let input = include_str!("../input.txt");
    let valve_lookup: ValveLookup = input.lines().map(parse_valve).collect();
    let shortest_paths =
        floyd_warshall_shortest_paths(&valve_lookup).filter_out_faulty_valves(&valve_lookup);

    let mut pruner = |thread_set: &[Rc<Thread>], total_runtime: u32, result: &mut u32| {
        let current_score = thread_set
            .iter()
            .map(|thread| thread.score(&valve_lookup, total_runtime))
            .sum::<u32>();

        let upper_bound = current_score
            + max_remaining_value(
                earliest_times_to_open_reachable_valves(thread_set, &shortest_paths, total_runtime),
                total_runtime,
                &valve_lookup,
            );

        if upper_bound <= *result {
            false
        } else {
            *result = u32::max(current_score, *result);
            true
        }
    };

    let mut part_1_answer = 0;
    pruning_search(
        &[Rc::new(Thread::Start)],
        &shortest_paths,
        30,
        &mut pruner,
        &mut part_1_answer,
    );

    println!("part 1: {part_1_answer}");

    let mut part_2_answer = 0;
    pruning_search(
        &[Rc::new(Thread::Start), Rc::new(Thread::Start)],
        &shortest_paths,
        26,
        &mut pruner,
        &mut part_2_answer,
    );

    println!("part 2: {part_2_answer}");
}
