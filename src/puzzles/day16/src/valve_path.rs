use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    iter::repeat_with,
};

use crate::{
    shortest_paths::ShortestPaths,
    valve_thread::{ThreadAction, ThreadCombinationSet, ValveThread},
    Valve, ValveLookup, MINUTES,
};

#[derive(Clone, Debug)]
pub struct ValvePath {
    pub done: bool,
    pub score_upper_bound: u32,
    pub threads: Vec<ValveThread>,
    pub score: u32,
    open_valves: HashSet<&'static str>,
}

impl ValvePath {
    pub fn new(
        start_valve: &Valve,
        shortest_paths: &ShortestPaths,
        valve_lookup: &ValveLookup,
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

        result.score_upper_bound =
            result.final_score_upper_bound(shortest_paths, minute, valve_lookup);

        result
    }

    pub fn all_possible_extensions(
        self,
        minute: u32,
        valve_lookup: &ValveLookup,
        shortest_paths: &ShortestPaths,
    ) -> BinaryHeap<ValvePath> {
        let all_thread_combinations = self
            .threads
            .into_iter()
            .map(|thread| thread.all_possible_extensions(&self.open_valves, valve_lookup))
            .fold(
                ThreadCombinationSet::new(),
                |thread_combination_set, thread_extensions| {
                    thread_combination_set.add_thread_extensions(thread_extensions)
                },
            );

        all_thread_combinations
            .candidates
            .into_iter()
            .map(|threads| {
                let mut opened_valve_counts = HashMap::new();
                for thread in threads.iter() {
                    if let Some(ThreadAction::OpenValve(opened_valve)) = thread.actions.last() {
                        opened_valve_counts
                            .entry(*opened_valve)
                            .and_modify(|x| *x += 1)
                            .or_insert(1);
                    }
                }
                (threads, opened_valve_counts)
            })
            .filter(|(_, opened_valve_counts)| !opened_valve_counts.iter().any(|(_, v)| *v > 1))
            .map(|(extended_threads, opened_valve_counts)| {
                let score = opened_valve_counts
                    .keys()
                    .fold(self.score, |acc, &valve_name| {
                        acc + valve_lookup.get(valve_name).unwrap().flow_rate * (MINUTES - minute)
                    });

                let mut path = ValvePath {
                    score,
                    done: extended_threads.iter().all(|thread| thread.done),
                    score_upper_bound: 0,
                    open_valves: extended_threads
                        .iter()
                        .flat_map(|thread| {
                            thread.actions.iter().filter_map(|action| match action {
                                ThreadAction::OpenValve(valve_name) => Some(*valve_name),
                                _ => None,
                            })
                        })
                        .collect(),
                    threads: extended_threads,
                };

                path.score_upper_bound =
                    path.final_score_upper_bound(shortest_paths, minute, valve_lookup);
                path
            })
            .collect()
    }

    fn final_score_upper_bound(
        &self,
        shortest_paths: &ShortestPaths,
        minute: u32,
        valve_lookup: &ValveLookup,
    ) -> u32 {
        let reachable_values = self.threads.iter().fold(HashMap::new(), |acc, thread| {
            thread
                .remaining_reachable_values(shortest_paths, &self.open_valves, minute, valve_lookup)
                .into_iter()
                .map(|(k, v)| {
                    let max_val = acc.get(k).map(|acc_v| u32::max(*acc_v, v)).unwrap_or(v);
                    (k, max_val)
                })
                .collect()
        });
        self.score + reachable_values.values().sum::<u32>()
    }
}

impl PartialEq for ValvePath {
    fn eq(&self, other: &Self) -> bool {
        self.score_upper_bound == other.score_upper_bound
    }
}

impl Eq for ValvePath {}

impl PartialOrd for ValvePath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score_upper_bound.partial_cmp(&other.score_upper_bound)
    }
}

impl Ord for ValvePath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score_upper_bound.cmp(&other.score_upper_bound)
    }
}
