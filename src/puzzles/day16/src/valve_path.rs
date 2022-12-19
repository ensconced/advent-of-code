use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{shortest_paths::ShortestPaths, valve_thread::ValveThread, Valve};

#[derive(Clone, Debug)]
pub struct ValvePath<'a> {
    pub done: bool,
    pub score_upper_bound: u32,
    pub thread: ValveThread<'a>,
    open_valves: HashSet<&'a str>,
}

impl<'a> ValvePath<'a> {
    pub fn new(
        start_valve: &'a Valve,
        shortest_paths: &ShortestPaths,
        valve_lookup: &'a HashMap<&'a str, Valve>,
    ) -> Self {
        let minute = 0;
        let open_valves = HashSet::new();

        let mut result = Self {
            thread: ValveThread::new(start_valve),
            open_valves,
            done: false,
            score_upper_bound: 0,
        };

        result.score_upper_bound = result.final_score_upper_bound(
            &result.open_valves,
            valve_lookup,
            shortest_paths,
            minute,
        );

        result
    }

    pub fn all_possible_extensions(
        self,
        minute: u32,
        valve_lookup: &'a HashMap<&str, Valve>,
        shortest_paths: &ShortestPaths,
    ) -> BinaryHeap<ValvePath<'a>> {
        self.thread
            .all_possible_extensions(valve_lookup, &self.open_valves, minute)
            .into_iter()
            .map(|extended_thread| {
                let mut path = ValvePath {
                    done: extended_thread.done,
                    score_upper_bound: 0,
                    open_valves: extended_thread.opened_valves.clone(),
                    thread: extended_thread,
                };
                path.score_upper_bound = path.final_score_upper_bound(
                    &path.open_valves,
                    valve_lookup,
                    shortest_paths,
                    minute,
                );
                path
            })
            .collect()
    }

    fn final_score_upper_bound(
        &self,
        open_valves: &HashSet<&str>,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
        minute: u32,
    ) -> u32 {
        let reachable_valve_values = self.thread.upper_bound_of_remaining_reachable_value(
            shortest_paths,
            open_valves,
            minute,
            valve_lookup,
        );
        self.thread.score + reachable_valve_values.values().sum::<u32>()
    }
}

impl<'a> PartialEq for ValvePath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.score_upper_bound == other.score_upper_bound
    }
}

impl<'a> Eq for ValvePath<'a> {}

impl<'a> PartialOrd for ValvePath<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score_upper_bound.partial_cmp(&other.score_upper_bound)
    }
}

impl<'a> Ord for ValvePath<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score_upper_bound.cmp(&other.score_upper_bound)
    }
}
