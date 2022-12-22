use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;

use crate::{shortest_paths::ShortestPaths, utils::combinations_taking_one_from_each, ValveLookup};

#[derive(Debug)]
pub enum Thread {
    Start,
    Extension {
        opened_valve: &'static str,
        minute_opened: u32,
        prev: Rc<Thread>,
    },
}

pub fn pruning_search<'a, F: FnMut(&[Rc<Thread>], u32, &mut u32) -> bool>(
    thread_set: &[Rc<Thread>],
    shortest_paths: &'a ShortestPaths,
    total_runtime: u32,
    is_potential_solution: &mut F,
    result: &mut u32,
) {
    if is_potential_solution(thread_set, total_runtime, result) {
        for extended_thread_set in extensions(thread_set, shortest_paths, total_runtime) {
            pruning_search(
                &extended_thread_set,
                shortest_paths,
                total_runtime,
                is_potential_solution,
                result,
            );
        }
    }
}

pub fn all_opened_valves(thread_set: &[Rc<Thread>]) -> HashSet<&'static str> {
    thread_set
        .iter()
        .map(|thread| thread.all_opened_valves())
        .reduce(|acc, x| acc.union(&x).cloned().collect())
        .unwrap_or_default()
}

fn extensions(
    thread_set: &[Rc<Thread>],
    shortest_paths: &ShortestPaths,
    total_runtime: u32,
) -> Vec<Vec<Rc<Thread>>> {
    let opened_valves = all_opened_valves(thread_set);

    let possible_extensions: Vec<Vec<_>> = thread_set
        .iter()
        .map(|thread| {
            let reachable_valves: HashMap<&str, u32> = thread
                .earliest_times_to_open_reachable_closed_valves(
                    shortest_paths,
                    total_runtime,
                    &opened_valves,
                )
                .into_iter()
                .filter(|(k, _)| !opened_valves.contains(k))
                .collect();
            reachable_valves
                .into_iter()
                .map(|(target, minute_opened)| {
                    Rc::new(Thread::Extension {
                        minute_opened,
                        opened_valve: target,
                        prev: thread.clone(),
                    })
                })
                .collect()
        })
        .collect();

    combinations_taking_one_from_each(possible_extensions)
        .into_iter()
        .filter(|extension_combination| {
            extension_combination
                .iter()
                .map(|thr| thr.current_valve())
                .all_unique()
        })
        .collect()
}

pub fn max_remaining_value(
    reachable_closed_valves: HashMap<&'static str, u32>,
    total_runtime: u32,
    valve_lookup: &ValveLookup,
) -> u32 {
    reachable_closed_valves
        .into_iter()
        .map(|(valve_name, minute_opened)| {
            let valve = valve_lookup.get(valve_name).unwrap();
            valve.flow_rate * (total_runtime - minute_opened)
        })
        .sum()
}

impl<'a> Thread {
    fn minute_opened(&self) -> u32 {
        match self {
            Self::Start => 0,
            Self::Extension { minute_opened, .. } => *minute_opened,
        }
    }

    fn all_opened_valves(&self) -> HashSet<&'static str> {
        match self {
            Self::Start => HashSet::new(),
            Self::Extension {
                opened_valve, prev, ..
            } => HashSet::from([*opened_valve])
                .union(&prev.all_opened_valves())
                .cloned()
                .collect(),
        }
    }

    pub fn score(&self, valve_lookup: &ValveLookup, total_runtime: u32) -> u32 {
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

    fn current_valve(&self) -> &'static str {
        match self {
            Thread::Start => "AA",
            Thread::Extension { opened_valve, .. } => opened_valve,
        }
    }

    pub fn earliest_times_to_open_reachable_closed_valves(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
        opened_valves: &HashSet<&'static str>,
    ) -> HashMap<&'static str, u32> {
        shortest_paths
            .all_shortest_paths_from(self.current_valve())
            .into_iter()
            .flatten()
            .filter_map(move |(target, path_length)| {
                let earliest_possible_minute_opened = self.minute_opened() + path_length + 1;
                let is_valid = earliest_possible_minute_opened < total_runtime
                    && !opened_valves.contains(target);
                is_valid.then_some((*target, earliest_possible_minute_opened))
            })
            .collect()
    }
}
