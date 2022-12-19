use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    iter::repeat_with,
};

use itertools::Itertools;

use crate::{
    shortest_paths::ShortestPaths,
    valve_thread::{ThreadAction, ValveThread},
    Valve,
};

#[derive(Clone, Debug)]
pub struct ValvePath<'a> {
    pub done: bool,
    pub score_upper_bound: u32,
    pub threads: Vec<ValveThread<'a>>,
    score: u32,
    open_valves: HashSet<&'a str>,
}

impl<'a> ValvePath<'a> {
    pub fn new(
        start_valve: &'a Valve,
        shortest_paths: &ShortestPaths,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        thread_count: usize,
    ) -> Self {
        let minute = 0;
        let open_valves = HashSet::new();

        let mut result = Self {
            score: 0,
            threads: repeat_with(|| ValveThread::new(start_valve))
                .take(thread_count)
                .collect(),
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
        let all_thread_possibilities = self
            .threads
            .iter()
            .map(|thread| thread.all_possible_extensions(valve_lookup, &self.open_valves, minute))
            .fold(Vec::<Vec<ValveThread>>::new(), |acc, x| {
                acc.into_iter()
                    .cartesian_product(x)
                    .map(|(threads, another_thread)| {
                        let combined_threads = threads.clone();
                        threads.push(another_thread);
                        combined_threads
                    })
                    .collect()
            });

        all_thread_possibilities
            .into_iter()
            .map(|threads| {
                let opened_valve_counts = HashMap::new();
                for thread in threads {
                    if let Some(ThreadAction::OpenValve(opened_valve)) = thread.actions.last() {
                        opened_valve_counts
                            .entry(opened_valve)
                            .and_modify(|x| *x = *x + 1)
                            .or_insert(1);
                    }
                }
                (threads, opened_valve_counts)
            })
            .filter(|(threads, opened_valve_counts)| opened_valve_counts.iter().any(|k, v| v > 1))
            .map(|(extended_threads, opened_valve_counts)| {
                // let score = if let Some(opened_valve_name) = e

                let mut path = ValvePath {
                    score,
                    done: extended_threads.iter().all(|thread| thread.done),
                    score_upper_bound: 0,
                    open_valves: extended_threads.iter().fold(HashSet::new(), |acc, thread| {
                        thread.opened_valves.union(&acc).cloned().collect()
                    }),
                    threads: extended_threads,
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
        let reachable_valve_values = self.threads.iter().fold(HashMap::new(), |acc, thread| {
            thread
                .upper_bound_of_remaining_reachable_value(
                    shortest_paths,
                    &self.open_valves,
                    minute,
                    valve_lookup,
                )
                .into_iter()
                .map(|(k, v)| {
                    let min_dist = acc.get(k).map(|acc_v| u32::min(*acc_v, v)).unwrap_or(v);
                    (k, min_dist)
                })
                .collect()
        });
        self.score + reachable_valve_values.values().sum::<u32>()
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
