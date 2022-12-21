use std::{borrow::Borrow, collections::HashMap, rc::Rc};

use crate::{shortest_paths::ShortestPaths, ValveLookup};

#[derive(Debug)]
pub enum Thread {
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

    pub fn pruning_search<F: FnMut(&Rc<Thread>) -> bool>(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
        is_potential_solution: &mut F,
    ) {
        if is_potential_solution(self) {
            for extension in self.extensions(shortest_paths, total_runtime) {
                extension.pruning_search(shortest_paths, total_runtime, is_potential_solution);
            }
        }
    }

    pub fn score_upper_bound(
        self: &Rc<Self>,
        shortest_paths: &'a ShortestPaths,
        total_runtime: u32,
        valve_lookup: &ValveLookup,
    ) -> u32 {
        let remaining_value: u32 = self
            .reachable_closed_valves(shortest_paths, total_runtime)
            .into_iter()
            .map(|(valve_name, minute_opened)| {
                let valve = valve_lookup.get(valve_name).unwrap();
                valve.flow_rate * (total_runtime - minute_opened)
            })
            .sum();
        remaining_value + self.score(valve_lookup, total_runtime)
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
                let earliest_possible_minute_opened = self.minute_opened() + path_length + 1;
                let is_valid =
                    earliest_possible_minute_opened < total_runtime && !self.valve_is_open(target);
                is_valid.then_some((*target, earliest_possible_minute_opened))
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
